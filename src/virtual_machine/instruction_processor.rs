use crate::sql_compilator::parser;
use crate::sql_compilator::tokenizer;

struct InstructionProcessor {
    instruction: parser::Instruction,
}

impl InstructionProcessor {
    pub fn new(&self, instruction: parser::Instruction) -> Self {
        InstructionProcessor { instruction }
    }

    pub fn process_instruction(&self) {
        match self.instruction.base_command {
            tokenizer::CommandType::CreateTable => self.create_table_file(),
            _ => todo!(),
        }
    }

    fn create_table_file(&self) {}
}
