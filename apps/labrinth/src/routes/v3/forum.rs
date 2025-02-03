use std::collections::HashMap;
use std::net::Ipv4Addr;
use crate::auth::{get_user_from_headers, AuthenticationError};
use crate::database::models::forum::PostBuilder;
use crate::database::models::forum::{Discussion, PostIndex};
use crate::database::models::ids::{DiscussionId, PostId};
use crate::database::models::notification_item::NotificationBuilder;
use crate::database::redis::RedisPool;
use crate::models::ids::base62_impl::parse_base62;
use crate::models::notifications::NotificationBody;
use crate::models::pats::Scopes;
use crate::queue::session::AuthQueue;

use crate::util::validate::validation_errors_to_string;
use crate::{
    database,
    models::v3::forum::{ForumResponse, PostResponse, PostsQueryParams},
    routes::ApiError,
};
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("forum")
            .route("", web::get().to(forums))
            .route("", web::post().to(forum_create))
            .route("{id}", web::get().to(forum_get))
            .route("{type}/lists", web::get().to(forums_get))
            .route("{id}/posts", web::get().to(posts_get))
            .route("{id}/post", web::post().to(posts_post)),
    );
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ForumRequest {
    #[validate(length(max = 300))]
    pub title: String,
    #[validate(length(max = 65536))]
    pub context: String,

    // 限制只能 chat 和 notice
    pub forum_type: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostRequest {
    #[validate(length(max = 65536))]
    pub content: String,
    pub replied_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ForumsQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn forum_get(
    _req: HttpRequest,
    info: web::Path<(String,)>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let discussion_id: String = info.into_inner().0;
    let discussion_id =
        DiscussionId(parse_base62(&discussion_id.to_string()).unwrap() as i64);
    let discussion = crate::database::models::forum::Discussion::get_id(
        discussion_id.0,
        &**pool,
        &redis,
    )
    .await?;
    if discussion.is_none() {
        return Err(ApiError::NotFound);
    }
    let discussion = discussion.unwrap();
    let response: ForumResponse = discussion.into();
    Ok(HttpResponse::Ok().json(json!(response)))
}

pub async fn forums(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {

    let mut exec = pool.acquire().await?;

    let forums =
        crate::database::models::forum::Discussion::forums(&mut *exec, &redis)
            .await?;

    let forums = forums.into_iter().map(|x| x.0).collect::<Vec<_>>();

    let forums = crate::database::models::forum::Discussion::get_many(
        &forums, &mut *exec, &redis,
    )
    .await?;
    let mut forums: Vec<ForumResponse> =
        forums.into_iter().map(|x| x.into()).collect::<Vec<_>>();

    forums.sort_by(|a, b| b.last_post_time.cmp(&a.last_post_time));

    Ok(HttpResponse::Ok().json(json!({
        "forums": forums,
    })))
}

pub async fn forums_get(
    _req: HttpRequest,
    info: web::Path<(String,)>,
    query: web::Query<ForumsQueryParams>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let forum_type: String = info.into_inner().0;

    let params = query.into_inner().clone();
    let page_size = params.page_size.unwrap_or(20) as i64;
    let page = params.page.unwrap_or(1) as i64;
    let mut exec = pool.acquire().await?;

    let forums = crate::database::models::forum::Discussion::get_forums(
        forum_type, &mut *exec, &redis,
    )
    .await?;
    let total = forums.len();
    let offset = ((page - 1) * page_size) as usize;

    let forums = forums
        .into_iter()
        .skip(offset)
        .take(page_size as usize)
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let forums = crate::database::models::forum::Discussion::get_many(
        &forums, &mut *exec, &redis,
    )
    .await?;
    let mut forums: Vec<ForumResponse> =
        forums.into_iter().map(|x| x.into()).collect::<Vec<_>>();

    forums.sort_by(|a, b| b.last_post_time.cmp(&a.last_post_time));

    Ok(HttpResponse::Ok().json(json!({
        "forums": forums,
        "pagination": {
            "total": total
        }
    })))
}

pub async fn posts_get(
    _req: HttpRequest,
    info: web::Path<(String,)>,
    query: web::Query<PostsQueryParams>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let discussion_id: String = info.into_inner().0;
    let params = query.into_inner().clone();
    let page_size = params.page_size.unwrap_or(20) as i64;
    let page = params.page.unwrap_or(1) as i64;
    let mut exec = pool.acquire().await?;
    let exec_ref = &mut *exec;
    let discussion_id =
        DiscussionId(parse_base62(&discussion_id.to_string()).unwrap() as i64);
    let discussion = crate::database::models::forum::Discussion::get_id(
        discussion_id.0,
        exec_ref,
        &redis,
    )
    .await?;
    if discussion.is_none() {
        return Err(ApiError::NotFound);
    }
    let mut forum_floor_numbers: Vec<PostIndex> = discussion.unwrap().posts;

    // 对forum_floor_numbers进行排序
    forum_floor_numbers.sort_by(|a, b| a.floor_number.cmp(&b.floor_number));

    // 在使用前克隆一份
    let forum_floor_numbers_clone: Vec<PostIndex> = forum_floor_numbers.clone();

    // 获取需要查询的楼层号
    let offset = ((page - 1) * page_size) as usize;
    let floor_numbers: Vec<PostIndex> = forum_floor_numbers
        .clone()
        .into_iter()
        .skip(offset)
        .take(page_size as usize)
        .collect();
    // 使用获取到的 floor_numbers 从 redis 查询完整的帖子信息
    let ids = &floor_numbers
        .iter()
        .map(|x| x.post_id.0)
        .collect::<Vec<i64>>();
    // 修改输出所有查询到的 floor_number
    let mut posts: Vec<PostResponse> =
        crate::database::models::forum::PostQuery::get_many(
            ids,
            &discussion_id,
            &**pool,
            &redis,
        )
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<PostResponse>>();

    // 对 posts 进行排序
    posts.sort_by(|a, b| a.floor_number.cmp(&b.floor_number));

    Ok(HttpResponse::Ok().json(json!({
        "posts": posts,
        "pagination": {
            "total": forum_floor_numbers_clone.len()
        }
    })))
}

pub async fn forum_create(
    req: HttpRequest,
    body: web::Json<ForumRequest>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
    session_queue: web::Data<AuthQueue>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|err| {
        ApiError::Validation(validation_errors_to_string(err, None))
    })?;

    let user_option = get_user_from_headers(
        &req,
        &**pool,
        &redis,
        &session_queue,
        Some(&[Scopes::PROJECT_READ, Scopes::VERSION_READ]),
    )
    .await
    .map(|x| x.1)
    .ok();
    if user_option.is_none() {
        return Err(ApiError::Authentication(
            AuthenticationError::InvalidCredentials,
        ));
    }

    // 检查用户是否绑定手机号
    if user_option.as_ref().unwrap().has_phonenumber.is_none()
        || !user_option.as_ref().unwrap().has_phonenumber.unwrap()
    {
        return Err(ApiError::InvalidInput(
            "请先绑定手机号，再进行回复".to_string(),
        ));
    }
    if body.forum_type.is_empty() {
        return Err(ApiError::InvalidInput("请选择帖子类型".to_string()));
    }
    if body.title.is_empty() {
        return Err(ApiError::InvalidInput("请输入帖子标题".to_string()));
    }
    if body.context.is_empty() {
        return Err(ApiError::InvalidInput("请输入帖子内容".to_string()));
    }

    let types = ["notice", "chat", "article"];
    if !types.contains(&body.forum_type.as_str()) {
        return Err(ApiError::InvalidInput("请选择正确的帖子类型".to_string()));
    }
    if body.forum_type == "notice"
        && user_option.as_ref().unwrap().role
            != crate::models::v3::users::Role::Admin
    {
        return Err(ApiError::InvalidInput(
            "只有管理员可以创建公告".to_string(),
        ));
    }
    // 检查帖子内容
    let risk = crate::util::risk::check_text_risk(
        &body.context,
        &user_option.as_ref().unwrap().username,
        &format!("/user/{}", user_option.as_ref().unwrap().username),
        "创建帖子",
        &redis,
    )
    .await?;
    if !risk {
        return Err(ApiError::InvalidInput(
                "帖子内容包含敏感词，已被记录该次提交，请勿在本网站使用涉及敏感词的帖子回复内容".to_string(),
            ));
    }

    let risk = crate::util::risk::check_text_risk(
        &body.title,
        &user_option.as_ref().unwrap().username,
        &format!("/user/{}", user_option.as_ref().unwrap().username),
        "创建帖子",
        &redis,
    )
    .await?;
    if !risk {
        return Err(ApiError::InvalidInput(
                "帖子标题包含敏感词，已被记录该次提交，请勿在本网站使用涉及敏感词的帖子回复内容".to_string(),
            ));
    }

    let mut transaction = pool.begin().await?;
    let discussion_id =
        crate::database::models::ids::generate_discussion_id(&mut transaction)
            .await?;

    let state = "open".to_string();
    
    // if body.forum_type == "article" {
    //
    // }
    
    let discussion = database::models::forum::Discussion {
        id: discussion_id,
        title: body.title.clone(),
        content: body.context.clone(),
        category: body.forum_type.clone(),
        created_at: chrono::Utc::now(),
        updated_at: None,
        user_id: database::models::UserId::from(
            user_option.as_ref().unwrap().id,
        ),
        organization_id: None,
        last_post_time: chrono::Utc::now(),
        state,
        pinned: false,
        deleted: false,
        deleted_at: None,
        user_name: user_option.as_ref().unwrap().username.clone(),
        avatar: user_option.as_ref().unwrap().avatar_url.clone(),
        project_id: None,
        organization: None,
    };
    discussion.insert(&mut transaction).await?;
    transaction.commit().await?;
    crate::database::models::forum::Discussion::clear_cache_discussions(
        &[discussion.category.clone(), "all".to_string()],
        &redis,
    )
    .await?;

    let id: crate::models::v3::forum::DiscussionId = discussion_id.into();

    Ok(HttpResponse::Ok().json(json!({
        "discussion": id
    })))
}

pub async fn posts_post(
    req: HttpRequest,
    info: web::Path<(String,)>,
    body: web::Json<PostRequest>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
    session_queue: web::Data<AuthQueue>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|err| {
        ApiError::Validation(validation_errors_to_string(err, None))
    })?;
    let string = info.into_inner().0;
    let discussion_id = DiscussionId(parse_base62(&string)? as i64);
    let discussion =
        Discussion::get_id(discussion_id.0, &**pool, &redis).await?;

    // 获取用户信息
    let user_option = get_user_from_headers(
        &req,
        &**pool,
        &redis,
        &session_queue,
        Some(&[Scopes::PROJECT_READ, Scopes::VERSION_READ]),
    )
    .await
    .map(|x| x.1)
    .ok();

    // 检查用户是否登录
    if user_option.is_none() {
        return Err(ApiError::Authentication(
            AuthenticationError::InvalidCredentials,
        ));
    }

    // 检查帖子是否存在
    if discussion.is_none() {
        return Err(ApiError::NotFound);
    }
    // 获取帖子
    let discussion_ = discussion.unwrap();
    let mut discussion = discussion_.clone().inner;

    // 检查帖子是否已关闭
    if discussion.state == "closed"
        && user_option.as_ref().unwrap().role
            != crate::models::v3::users::Role::Admin
    {
        return Err(ApiError::InvalidInput("帖子已关闭，无法回复".to_string()));
    }

    // 检查帖子是否为公告
    if discussion.category == "notice"
        && user_option.as_ref().unwrap().role
            != crate::models::v3::users::Role::Admin
    {
        return Err(ApiError::InvalidInput("公告帖子无法回复".to_string()));
    }

    // 检查用户是否绑定手机号
    if user_option.as_ref().unwrap().has_phonenumber.is_none()
        || !user_option.as_ref().unwrap().has_phonenumber.unwrap()
    {
        return Err(ApiError::InvalidInput(
            "请先绑定手机号，再进行回复".to_string(),
        ));
    }

    // 检查回复内容
    let risk = crate::util::risk::check_text_risk(
        &body.content,
        &user_option.as_ref().unwrap().username,
        &format!("/d/{}", string),
        "创建帖子回复",
        &redis,
    )
    .await?;
    if !risk {
        return Err(ApiError::InvalidInput(
            "帖子回复内容包含敏感词，已被记录该次提交，请勿在本网站使用涉及敏感词的帖子回复内容".to_string(),
        ));
    }

    let mut transaction = pool.begin().await?;

    let post_id: PostId =
        crate::database::models::ids::generate_post_id(&mut transaction)
            .await?;
    let id: crate::models::v3::forum::DiscussionId = discussion_id.into();
    let post = PostBuilder {
        id: post_id,
        discussion_id,
        content: body.content.clone(),
        created_at: chrono::Utc::now(),
        user_id: database::models::UserId::from(
            user_option.as_ref().unwrap().id,
        ),
        replied_to: body
            .replied_to
            .clone()
            .map(|x| parse_base62(&x).unwrap() as i64),
    };
    discussion.last_post_time = chrono::Utc::now();
    discussion.update_last_post_time(&mut transaction).await?;
    post.insert(&mut transaction).await?;
    let number_of_posts = discussion_.posts.len() + 1;
    let number_of_posts = number_of_posts as u32;
    // 发送通知
    let notification = NotificationBuilder {
        body: NotificationBody::Forum {
            forum_id: id,
            forum_title: discussion.title.clone(),
            forum_type: discussion.category.clone(),
            number_of_posts,
            project_id: discussion.project_id.map(|x| x.into()),
            sender: user_option.as_ref().unwrap().username.clone(),
        },
    };
    notification
        .insert(discussion.user_id, &mut transaction, &redis)
        .await?;

    transaction.commit().await?;

    crate::database::models::forum::Discussion::clear_cache(
        &[discussion_id],
        &redis,
    )
    .await?;
    crate::database::models::forum::Discussion::clear_cache_discussions(
        &[discussion.category.clone(), "all".to_string()],
        &redis,
    )
    .await?;

    let posts: Vec<PostResponse> =
        crate::database::models::forum::PostQuery::get_many(
            &[post_id.0],
            &discussion_id,
            &**pool,
            &redis,
        )
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<PostResponse>>();

    // info!("d: {:?}", discussion);
    Ok(HttpResponse::Ok().json(json!({
        "post": posts.first().unwrap()
    })))
}
