use anyhow::Result;
use baad::utils::FileManager;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fs, io};
use baad_core::{info, success, warn};
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
        format!("{}{}", base_name, if Self::is_windows() { ".exe" } else { "" })
    }

    fn extract_zip<R: Read + Seek>(archive: &mut ZipArchive<R>, target_dir: &Path, binary_name: &str) -> Result<()> {
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

    pub fn il2cpp_dumper(&self, forced: bool) -> Result<PathBuf> {
        let target_dir = self
            .file_manager
            .get_data_path("tools/Il2CppInspectorRedux");
        let binary_name = Self::get_binary_name("Il2CppInspector.Redux.CLI");
        let binary_path = target_dir.join(&binary_name);

        if binary_path.exists() && !forced {
            warn!("Il2CppInspectorRedux already extracted, skipping...");
            return Ok(binary_path);
        }

        info!("Extracting Il2CppInspectorRedux...");

        let il2cpp_dumper = self
            .file_manager
            .get_data_path("tools/Il2CppInspectorRedux.zip");

        fs::create_dir_all(&target_dir)?;
        let file = fs::File::open(il2cpp_dumper)?;
        let mut archive = ZipArchive::new(file)?;

        Self::extract_zip(&mut archive, &target_dir, &binary_name)?;

        Ok(binary_path)
    }

    pub fn fbs_dumper(&self, forced: bool) -> Result<PathBuf> {
        let target_dir = self.file_manager.get_data_path("tools/FbsDumperV2");
        let binary_name = Self::get_binary_name("FbsDumper");
        let binary_path = target_dir.join(&binary_name);

        if binary_path.exists() && !forced {
            warn!("FbsDumperV2 already extracted, skipping...");
            return Ok(binary_path);
        }

        info!("Extracting FbsDumperV2...");

        let fbs_dumper = self.file_manager.get_data_path("tools/FbsDumperV2.zip");

        fs::create_dir_all(&target_dir)?;
        let file = fs::File::open(fbs_dumper)?;
        let mut outer_archive = ZipArchive::new(file)?;

        let mut inner_zip = outer_archive.by_index(0)?;
        let mut inner_zip_data = Vec::new();
        io::copy(&mut inner_zip, &mut inner_zip_data)?;

        let mut inner_archive = ZipArchive::new(io::Cursor::new(inner_zip_data))?;

        Self::extract_zip(&mut inner_archive, &target_dir, &binary_name)?;

        Ok(binary_path)
    }

    pub fn flatc(&self, forced: bool) -> Result<PathBuf> {
        let target_dir = self
            .file_manager
            .get_data_path("tools/Flatc");
        let binary_name = Self::get_binary_name("flatc");
        let binary_path = target_dir.join(&binary_name);

        if binary_path.exists() && !forced {
            warn!("Flatc already extracted, skipping...");
            return Ok(binary_path);
        }

        info!("Extracting Flatc...");

        let flatc = self
            .file_manager
            .get_data_path("tools/Flatc.zip");

        fs::create_dir_all(&target_dir)?;
        let file = fs::File::open(flatc)?;
        let mut archive = ZipArchive::new(file)?;

        Self::extract_zip(&mut archive, &target_dir, &binary_name)?;

        Ok(binary_path)
    }
}