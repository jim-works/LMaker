use super::code_generator;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const parser_class_name: &str = "Parser";
const program_class_name: &str = "Program";

pub struct CodeGeneratorCSharp {
    program: File,
    parser: File,
}

impl code_generator::CodeGenerator for CodeGeneratorCSharp {
    fn generate(mut self, data: &code_generator::CodeGeneratorData, dest_folder: &Path) {
        generate_csproj(
            dest_folder
                .join(data.project_name)
                .join(".csproj")
                .as_path(),
        );
    }
}

fn generate_csproj(path: &Path) {
    let mut file = match File::create(path) {
        Err(e) => panic!("couldn't create file {}, error: {}", path.display(), e),
        Ok(f) => f,
    };
    match file.write_all(b"<Project Sdk=\"Microsoft.NET.Sdk\">\n\t<PropertyGroup>\n\t\t<OutputType>Exe</OutputType>\n\t\t<TargetFramework>netcoreapp3.1</TargetFramework>\n\t</PropertyGroup>\n</Project>") {
            Err(e) => panic!("couldn't write to file {}, error: {}", path.display(), e),
            _ => ()
        };
}

fn generate_program(path: &Path) {}
