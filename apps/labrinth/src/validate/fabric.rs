use crate::validate::{
    filter_out_packs, SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;

pub struct FabricValidator;

impl super::Validator for FabricValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["jar"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["fabric"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        if dotenvy::var("DEV")
            .ok()
            .and_then(|x| x.parse::<bool>().ok())
            .unwrap_or(false)
        {
            return Ok(ValidationResult::Pass);
        }
        if archive.by_name("manifest.json").is_ok() {
            return Ok(ValidationResult::Pass);
        }
        if archive.by_name("mod.json").is_ok() {
            return Ok(ValidationResult::Pass);
        }
        if archive.by_name("fabric.mod.json").is_err() {
            return Ok(ValidationResult::Warning(
                "未找到 fabric.mod.json 文件。提示：确保 fabric.mod.json 位于 Fabric 文件的根目录中！",
            ));
        }

        filter_out_packs(archive)?;

        Ok(ValidationResult::Pass)
    }
}
