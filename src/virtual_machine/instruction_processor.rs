use crate::config::Config;
use crate::config::get_project_root;
use crate::utils::file_handler;
use std::path::Path;
use std::path::PathBuf;

use crate::sql_compilator::parser;
use crate::sql_compilator::tokenizer;

pub struct InstructionProcessor<'db> {
    instruction: parser::Instruction,
    config: Config,
    tables_dir: PathBuf,
    database: &'db mut file_handler::Database,
}

impl<'db> InstructionProcessor<'db> {
    pub fn new(
        instruction: parser::Instruction,
        config_path: &Path,
        database: &'db mut file_handler::Database,
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

    pub fn process_instruction(&mut self) -> std::io::Result<()>{
        match self.instruction.base_command {
            tokenizer::CommandType::CreateTable => self.create_table_file(),
            _ => todo!(),
        }
    }

    fn create_table_file(&mut self) -> std::io::Result<()> {
        log::debug!("{:#?}", self.instruction);
        let owned_columns = self.instruction.columns.clone();
        self.database
            .create_table(&self.instruction.target_table, owned_columns)
    }
}
