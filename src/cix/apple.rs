use std::path::{Path,PathBuf};
use structopt::StructOpt;
use ci::common::CIResult;


/// Tools for Apple platforms.
#[derive(StructOpt)]
pub enum Apple {
    /// Prints all existing per-arch modules for both .app and .dSYM files.
    PrintModuleList(AppDSYMPair),
    /// Cross-checks for missing .dSYM files against to all code files in a .app package.
    MissingDSYMCheck(AppDSYMPair),
}
#[derive(StructOpt)]
pub struct AppDSYMPair {
    /// Path to .app directory or .ipa file.
    app: PathBuf,
    /// Path to .dSYM directory or .dSYM.zip file.
    dsym: PathBuf
}

pub async fn run(opt: &Apple) -> CIResult<()> {
    use Apple::*;
    match opt {
        PrintModuleList(opt) => print_module_list(&opt.app, &opt.dsym).await?,
        MissingDSYMCheck(opt) => missing_dsym_check(&opt.app, &opt.dsym).await?,
    }
    Ok(())
}

#[derive(serde::Serialize, Debug)]
struct Report {
    app: Vec<ci::apple::macho::File>,
    dsym: Vec<ci::apple::macho::File>,
}

async fn print_module_list(app: &Path, dsym: &Path) -> CIResult<()> {
    let report = Report {
        app: ci::apple::macho::scan_uuids(app)?,
        dsym: ci::apple::macho::scan_uuids(dsym)?,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
async fn missing_dsym_check(app: &Path, dsym: &Path) -> CIResult<()> {
    let app_bin_files = ci::apple::macho::scan_uuids(app)?;
    let dsym_bin_files = ci::apple::macho::scan_uuids(dsym)?;
    // Build required arch-bin map.
    type Bin = ci::apple::macho::MachO;
    let mut object_filepath_map = std::collections::HashMap::<Bin,PathBuf>::new();
    for app_bin_file in app_bin_files.iter() {
        for archbin in app_bin_file.bins.iter() {
            object_filepath_map.insert(archbin.clone(), app_bin_file.path.clone());
        }
    }
    // Remove discovered arch-bin one by one.
    for dsym_bin_file in dsym_bin_files.iter() {
        for archbin in dsym_bin_file.bins.iter() {
            object_filepath_map.remove(&archbin);
        }
    }
    let s = serde_json::to_string_pretty(&object_filepath_map)?;
    println!("{}", &s);
    Ok(())
}