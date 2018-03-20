extern crate walkdir;
extern crate protoc_grpcio;
extern crate protobuf;
extern crate grpcio;
extern crate futures;

pub mod proto;

// use std::io;
// use std::path::Path;
// use walkdir::WalkDir;

// pub fn generate_proto(proto_dir: &Path) {
//     for dir in WalkDir::new(proto_dir) {
//         match dir {
//             Err(err) => {
//                 let path = err.path().unwrap_or(Path::new("")).display();
//                 println!("failed to access entry {}", path);

//                 if let Some(inner) = err.io_error() {
//                     match inner.kind() {
//                         io::ErrorKind::InvalidData => {
//                             println!(
//                                 "entry contains invalid data: {}",
//                                 inner)
//                         }
//                         io::ErrorKind::PermissionDenied => {
//                             println!(
//                                 "Missing permission to read entry: {}",
//                                 inner)
//                         }
//                         _ => {
//                             println!(
//                                 "Unexpected error occurred: {}",
//                                 inner)
//                         }
//                     }
//                 }
//             }

//             Ok(entry) => {
//                 let path = entry.path();
//                 let metadata = entry.metadata().unwrap();
//                 if metadata.is_file() {
//                     println!("{:?}", path.display());
//                     println!("{:?}", path.parent().unwrap());
//                     protoc_grpcio::compile_grpc_protos(
//                         &[path],
//                         &[path.parent().unwrap()],
//                         &path.parent().unwrap()
//                     ).unwrap();
//                 }
//             },
//         }
//     }
// }
