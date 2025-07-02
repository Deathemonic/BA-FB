mod cli;
mod wrappers;
mod tools;

use crate::tools::fetch::ToolsFetcher;
use crate::tools::extract::ToolsExtractor;

use anyhow::Result;
use baad::utils::FileManager;

#[tokio::main]
async fn main() -> Result<()>{
    let file_manager = FileManager::new()?;
    let tools_fetcher = ToolsFetcher::new(file_manager.clone())?;
    
    tools_fetcher.il2cpp_dumper().await?;
    tools_fetcher.fbs_dumper().await?;

    let tools_extractor = ToolsExtractor::new(file_manager.clone())?;
    tools_extractor.il2cpp_dumper()?;
    tools_extractor.fbs_dumper()?;

    Ok(())
}
