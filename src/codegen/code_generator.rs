use crate::*;

pub struct CodeGeneratorData<'a> {
    pub table: parsing::parse_table::Table<'a>,
    pub project_name: &'a String,
}

pub trait CodeGenerator {
    fn generate(mut self, data: &CodeGeneratorData, dest_folder: &std::path::Path);
}
