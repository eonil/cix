use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fmt;
use serde::Serialize;
use crate::common::CIResult;
use crate::common::UUID;
use crate::common::*;


#[derive(Serialize, Debug)]
#[derive(Clone)]
pub struct File {
    pub path: PathBuf,
    pub bins: Vec<MachO>,
}

/// Mach-O binary information.
/// - `uuid`: Build UUID. Not so much thing about this value is known.
#[derive(Eq, PartialEq, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[derive(Serialize, Debug)]
pub struct MachO {
    pub arch: Arch,
    pub uuid: UUID,
}

#[derive(Eq, PartialEq, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Arch {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
}

/// Scans all executable files in an .app directory (or .ipa file). 
/// - returns: All discovered UUIDs for all files.
/// 
/// .dSYM files are also in Mach-O format. 
/// Therefore .dSYM files also can be scanned with this function.
/// 
pub fn scan_uuids(app: &Path) -> CIResult<Vec<File>> {
    let mut result = Vec::<File>::new();
    let pattern = app.to_str().ok_or(MissingError)?.appending("/**/*");
    for entry in glob::glob(&pattern)? {
        let path = entry?;
        let machos = match scan_uuids_in_executable_file(&path) {
            Err(_) => continue,
            Ok(machos) => machos,
        };
        if !machos.is_empty() {
            let file = File {
                path: path,
                bins: machos,
            };
            result.push(file);
        }
    }
    Ok(result)
}

fn scan_uuids_in_executable_file(exe: &Path) -> CIResult<Vec<MachO>> {
    use goblin::Object;
    use goblin::mach::Mach::*;
    let content = fs::read(exe)?;
    match Object::parse(&content[..])? {
        Object::Mach(Fat(mmach)) => scan_uuids_from_mmarch(&mmach),
        Object::Mach(Binary(macho)) => scan_uuids_from_macho(&macho),
        _ => Err("unsupported object file type.".into()),
    }
}

fn scan_uuids_from_mmarch(mmach: &goblin::mach::MultiArch) -> CIResult<Vec<MachO>> {
    let n = mmach.narches;
    let mut uuids = Vec::<MachO>::new();
    for i in 0..n {
        let macho = mmach.get(i)?;
        let uuid = scan_uuid_from_macho(&macho)?;
        uuids.push(uuid);
    }
    Ok(uuids)
}

fn scan_uuids_from_macho(macho: &goblin::mach::MachO) -> CIResult<Vec<MachO>> {
    let uuid = scan_uuid_from_macho(macho)?;
    Ok(vec![uuid])
}

fn scan_uuid_from_macho(macho: &goblin::mach::MachO) -> CIResult<MachO> {
    for lcmd in macho.load_commands.iter() {
        use goblin::mach::load_command::CommandVariant::*;
        match lcmd.command {
            Uuid(uuid_cmd) => { 
                let result = MachO {
                    arch: Arch {
                        cpu_type: macho.header.cputype(),
                        cpu_subtype: macho.header.cpusubtype(),
                    },
                    uuid: UUID::with(uuid_cmd.uuid.clone()),
                };
                return Ok(result);
            },
            _ => continue,
        }
    }
    return Err("missing LC_UUID command.".into());
}


























/////////////////////////////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for MachO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arch: {}, uuid: {}", self.arch, self.uuid)
    }
}
impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = goblin::mach::constants::cputype::get_arch_name_from_types(self.cpu_type, self.cpu_subtype).unwrap_or("<unknown>");
        write!(f, "{}", s)
    }
}








/////////////////////////////////////////////////////////////////////////////////////////////////////////

impl serde::Serialize for Arch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let s = goblin::mach::constants::cputype::get_arch_name_from_types(self.cpu_type, self.cpu_subtype).unwrap_or("<unknown>");
        serializer.serialize_str(&s)
    }
}


























/////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::super::super::common::*;
    use std::fs;
    use std::path::*;

    #[test]
    fn test_scanning_with_sample2() -> CIResult<()> {
        let sample2 = root().expect("cannot find srcroot").appending("test").appending("apple").appending("sample2");
        assert_eq!(sample2.exists(), true, "missing `sample2` directory.");

        let tmproot = sample2.appending(".tmp");
        let prodroot = sample2.appending(".build");
        if tmproot.exists() { fs::remove_dir_all(&tmproot)? }
        if prodroot.exists() { fs::remove_dir_all(&prodroot)? }
        fs::create_dir(&tmproot)?;
        fs::create_dir(&prodroot)?;
        let cmd = format!("cd {}; ./run", sample2.clone().into_os_string().into_string().unwrap());
        let out = std::process::Command::new("bash").args(vec!["-c", &cmd]).output()?;
        assert_eq!(out.status.success(), true, "{:?}", out);
        {
            let appbin = sample2.appending(".build/Sample2.app/Sample2");
            let specs = super::scan_uuids_in_executable_file(&appbin)?;
            assert_eq!(specs.len(), 2);
            assert_eq!(specs[0].arch.cpu_type, goblin::mach::constants::cputype::CPU_TYPE_X86_64);
            assert_eq!(specs[0].uuid, UUID::with([
                0xf8, 0xf8, 0xd5, 0xf1, 0x8f, 0x5d, 0x37, 0xc6, 
                0x97, 0x7e, 0x37, 0xbb, 0x2a, 0x4d, 0x11, 0x97,    
            ]));
            assert_eq!(specs[1].arch.cpu_type, goblin::mach::constants::cputype::CPU_TYPE_ARM64);
            assert_eq!(specs[1].uuid, UUID::with([
                0x86, 0x0c, 0x43, 0x60, 0xcd, 0x3c, 0x35, 0x90,
                0xaf, 0x4e, 0xe7, 0x59, 0x00, 0xbd, 0xee, 0xc4,
            ]));
        }
        fs::remove_dir_all(&tmproot)?;
        fs::remove_dir_all(&prodroot)?;
        Ok(())   
    }
    fn root() -> Option<PathBuf> {
        let p = Path::new(file!()).parent()?.parent()?.parent()?.parent()?.to_path_buf();
        Some(p)
    }
}

