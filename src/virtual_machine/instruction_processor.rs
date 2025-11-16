use crate::config::Config;
use crate::config::get_project_root;
use crate::utils::file_handler;
use std::path::Path;
use std::path::PathBuf;

use crate::sql_compilator::parser;
use crate::sql_compilator::tokenizer;

pub struct InstructionProcessor {
    instruction: parser::Instruction,
    config: Config,
    tables_dir: PathBuf,
    database: &file_handler::Database,
}

impl InstructionProcessor {
    pub fn new(
        instruction: parser::Instruction,
        config_path: &Path,
        database: &file_handler::Database,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load(config_path)?;
        let tables_dir = get_project_root().join(&config.database.url);
        Ok(InstructionProcessor {
            instruction,
            config,
            tables_dir,
            database,
        })
    }

    pub fn process_instruction(&self) {
        match self.instruction.base_command {
            tokenizer::CommandType::CreateTable => self.create_table_file(),
            _ => todo!(),
        }
    }

    fn create_table_file(&self) {
        log::debug!("{:#?}", self.instruction);
        todo!("Not implemented");
    }
}
