use std::fs;
use std::path::PathBuf;
use pbjson_build;

fn main() {
    build_config();
    //build_cni();
}

/*
/*
prost_build::Config::new()
    // Save descriptors to file
    .file_descriptor_set_path(&descriptor_path)
    // Override prost-types with pbjson-types
    .compile_well_known_types()
    .extern_path(".google.protobuf", "::pbjson_types")
    // Generate prost structs
    .compile_protos(&proto_files, &[root])?;

 */

 */

fn build_config(){
    let search_paths = vec![
        "../go/src/k8s.io/apimachinery/pkg/api/resource",
        "../go/src/k8s.io/apimachinery/pkg/runtime",
        "../go/src/k8s.io/apimachinery/pkg/apis/meta/v1",
        "../go/src/k8s.io/api/core/v1/",
        "../go/src/ssd-git.juniper.net/contrail/cn2/contrail/pkg/apis/core/v4"
    ];
    let out_dir = String::from("./src/protos");
    let mut protos = Vec::new();
    let mut includes = Vec::new();
    search_files_with_extension(search_paths, &mut protos, &mut includes);
   
    let incl = vec!["../go/src".to_owned()];
    let descriptor_path = PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("proto_descriptor.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .out_dir(&out_dir)
        //.include_file("mod.rs")
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        //.field_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(&protos, &incl)
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    let descriptor_set = std::fs::read(descriptor_path).unwrap();

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.api.core.v1"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.api.networking.v1"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.apimachinery.pkg.api.resource"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.apimachinery.pkg.apis.meta.v1"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.apimachinery.pkg.runtime"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .ignore_unknown_fields()
        .build(&[".k8s.io.apimachinery.pkg.util.intstr"]).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set).unwrap()
        .out_dir(&out_dir)
        .preserve_proto_field_names()
        .build(&[".ssd_git.juniper.net.contrail.cn2.contrail.pkg.apis.core.v4"]).unwrap();
}

fn search_files_with_extension(search_paths: Vec<&str>, protos: &mut Vec<String>, includes: &mut Vec<String>) {
    for dir in search_paths {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        search_files_with_extension(vec![path.to_str().unwrap()], protos, includes);
                    } else if path.is_file() {
                        if path.extension().map(|ext| ext == "proto").unwrap_or(false) {
                            protos.push(path.display().to_string());
                            includes.push(path.parent().unwrap().display().to_string());
                        }
                    }
                }
            }
        }
    }
}
