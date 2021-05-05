// use super::common::*;
// use std::path;
// use std::fs;
// 
// pub fn scanUUID(path: &path::Path) -> Result<UUID, &str> { 
//     use goblin::*;
//     let content = fs::read(path).map_err(|err| "")?;
//     match Object::parse(&content[..]) {
//         Object::Mach(Fat(macho)) => ,
//         Object::Mach(Binary(macho)) => ,
//         _ => Err(""),
//     }
// }
// fn scanUUID(header: goblin::mach::header::Header64) -> Result<UUID, &str> {
//     return header.
// }