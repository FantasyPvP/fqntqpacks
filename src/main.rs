use std::{fs, io::{self, Error, ErrorKind}, path::{Path, PathBuf}};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;
use walkdir::WalkDir;
use zip_archive::{Archiver, get_dir_list};
use num_cpus;

pub mod utils;

const PACK_REPO: &str = "./";
const RESOURCEPACKS: &str = "./_resourcepacks";

const IGNORE_FOLDERS_PREFIX: &str = "_";

fn main() {
    let conf = utils::load_config().unwrap();
    let pack_repo = PathBuf::from(PACK_REPO);
    clear_resourcepacks().unwrap();
    scan_directory(pack_repo).unwrap();
}

fn clear_resourcepacks() -> io::Result<()> {
    println!("[removing existing packs]");
    fs::remove_dir_all(RESOURCEPACKS)?;
    fs::create_dir(RESOURCEPACKS)?;
    println!(" => cleared resourcepacks folder");
    Ok(())
}

fn scan_directory(path: PathBuf) -> io::Result<()> {
    let mut pack_mcmeta = path.clone();
    pack_mcmeta.push("pack.mcmeta");
    if pack_mcmeta.exists() {
        // if pack metadata exists, put the pack into a zip folder.
        get_archive(path).map_err(|_| Error::new(ErrorKind::Other, "failed to archive zip"))?;
        return Ok(())
    } else {
        // recursively scans subdirectories for other packs
        for entry in fs::read_dir(path)? {
            let entry = entry?;

            if entry.path()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .map_err(|_| Error::new(ErrorKind::Other, "failed to convert os_str to string"))?
                .starts_with("_") {
                println!("found _");
                continue;
            }
            if let Some(zip_str) = entry.path().extension() {
                if zip_str == "zip" {
                    fs::copy(entry.path(), format!("{}/{}", RESOURCEPACKS, entry.path().file_name().unwrap().to_os_string().into_string().map_err(|_| Error::new(ErrorKind::Other, "failed to convert os_str to string"))?))?;
                }
            }
                
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    scan_directory(entry.path())?;
                }
            }    
        }
    }
    Ok(())
}

fn get_archive(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("CREATING ARCHIVE: {}", &path.display());
    if let Some(f) = path.file_stem() {
        let mut filename = f.to_os_string().into_string().map_err(|_| Error::new(ErrorKind::Other, "failed to convert os_str to string"))?;
        filename.push_str("_0");
        loop {
            if !Path::new(&format!("{}/{}.zip", RESOURCEPACKS, filename.to_string())).exists() { // checks if zip file already exists
                let zip_file = fs::File::create(format!("{}/{}.zip", RESOURCEPACKS, filename))?;
                let mut zip_writer = ZipWriter::new(zip_file);

                let options = FileOptions::default()
                    .compression_method(CompressionMethod::Deflated)
                    .unix_permissions(0o755);

                for entry in WalkDir::new(path.clone()).into_iter().filter_map(|e| e.ok()) {
                    let fpath = entry.path();
                    if fpath.is_file() {
                        let mut file = fs::File::open(fpath)?;
                        let relpath = fpath.strip_prefix(path.clone())?;
                        let mut zip_path = PathBuf::new();
                        zip_path.push(relpath);

                        zip_writer.start_file(zip_path.to_string_lossy().into_owned(), options)?;
                        std::io::copy(&mut file, &mut zip_writer)?;
                    }
                }
                zip_writer.finish()?;
                break;
                
            } else {
                // 10 increments the last digit of the filename by 1
                // TODO: add support for more than one digit
                let i: u32 = filename.pop().unwrap().to_digit(10).unwrap();
                filename.push(char::from_digit(i+1, 10).unwrap());
            }
        }
    }
    Ok(())
}