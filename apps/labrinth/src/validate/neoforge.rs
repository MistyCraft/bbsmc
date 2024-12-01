use crate::validate::{
    filter_out_packs, SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;

pub struct NeoForgeValidator;

impl super::Validator for NeoForgeValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["jar", "zip"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["neoforge"]
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
        if archive.by_name("META-INF/mods.toml").is_err()
            && archive.by_name("META-INF/neoforge.mods.toml").is_err()
            && archive.by_name("META-INF/MANIFEST.MF").is_err()
            && !archive.file_names().any(|x| x.ends_with(".class"))
        {
            return Ok(ValidationResult::Warning(
                "NeoForge 文件中没有 neoforge.mods.toml、mods.toml 或有效的类文件。",
            ));
        }

        filter_out_packs(archive)?;

        Ok(ValidationResult::Pass)
    }
}