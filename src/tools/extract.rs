use crate::helpers::config::*;

use anyhow::Result;
use baad::utils::FileManager;
use baad_core::{info, success, warn};
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fs, io};
use zip::ZipArchive;

pub struct ToolsExtractor {
    file_manager: Rc<FileManager>,
}

impl ToolsExtractor {
    pub fn new(file_manager: Rc<FileManager>) -> Result<Self> {
        Ok(Self { file_manager })
    }

    fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }

    fn make_executable(path: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms)?;
        }
        Ok(())
    }

    fn get_binary_name(base_name: &str) -> String {
        format!(
            "{}{}",
            base_name,
            if Self::is_windows() { ".exe" } else { "" }
        )
    }

    fn extract_zip<R: Read + Seek>(
        archive: &mut ZipArchive<R>,
        target_dir: &Path,
        binary_name: &str,
    ) -> Result<()> {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = target_dir.join(file.name());
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;

            if !Self::is_windows() && file.name().ends_with(binary_name) {
                Self::make_executable(&outpath)?;
            }
        }

        success!("Successfully extracted.");
        Ok(())
    }

    fn extract_tool(
        &self,
        binary_name: &str,
        zip_file: &str,
        path: &str,
        forced: bool,
    ) -> Result<PathBuf> {
        let tool_name = Path::new(zip_file)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(zip_file);

        let target_path = self
            .file_manager
            .get_data_path(&format!("{}/{}", TOOLS_DIR, path));
        let binary_name_with_ext = Self::get_binary_name(binary_name);
        let binary_path = target_path.join(&binary_name_with_ext);

        if binary_path.exists() && !forced {
            warn!("{} already extracted, skipping...", tool_name);
            return Ok(binary_path);
        }

        info!("Extracting {}...", tool_name);

        let zip_path = self
            .file_manager
            .get_data_path(&format!("{}/{}", TOOLS_DIR, zip_file));

        fs::create_dir_all(&target_path)?;
        let file = fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        Self::extract_zip(&mut archive, &target_path, &binary_name_with_ext)?;

        Ok(binary_path)
    }

    pub fn il2cpp_dumper(&self, forced: bool) -> Result<PathBuf> {
        self.extract_tool(IL2CPP_BINARY, IL2CPP_FILE, IL2CPP_DIR, forced)
    }

    pub fn fbs_dumper(&self, forced: bool) -> Result<PathBuf> {
        self.extract_tool(FBS_DUMPER_BINARY, FBS_DUMPER_FILE, FBS_DUMPER_DIR, forced)
    }

    pub fn flatc(&self, forced: bool) -> Result<PathBuf> {
        self.extract_tool(FLATC_BINARY, FLATC_FILE, FLATC_DIR, forced)
    }
}
