use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::sql_compilator::parser::ParsingError;

// Structure of file tree :
// mydb/
//   ├── metadata.ron          (metadata as ron file)
//   └── tables/
//       ├── users.meta.ron    (table schema)
//       ├── users.data.bin    (data as binary)
//       └── users.idx.bin     (index)

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseMetadata {
    name: String,
    version: String,
    tables: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TableMetadata {
    name: String,
    columns: Vec<Column>,
    row_count: u64,
    page_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub values: Vec<DataType>,
    pub is_primary_key: bool,
    // TODO: nullable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    Float,
    Integer,
    Text,
    Bool,
    Uuid,
}

impl DataType {
    pub fn from_string(data_type: String) -> Result<DataType, ParsingError> {
        match data_type.to_lowercase().as_str() {
            "float" => Ok(DataType::Float),
            "integer" => Ok(DataType::Integer),
            "text" => Ok(DataType::Text),
            "bool" => Ok(DataType::Bool),
            "uuid" => Ok(DataType::Uuid),
            &_ => Err(ParsingError::UnexpectedDataTypeProvided { found: data_type }),
        }
    }
}

pub struct Database {
    path: PathBuf,
    metadata: DatabaseMetadata,
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn create(path: &str, name: &str) -> io::Result<Self> {
        let db_path = PathBuf::from(path);
        std::fs::create_dir_all(&db_path)?;
        std::fs::create_dir_all(db_path.join("tables"))?;

        let metadata = DatabaseMetadata {
            name: name.to_string(),
            version: String::from("1.0"),
            tables: Vec::new(),
        };

        let ron = ron::ser::to_string_pretty(&metadata, Default::default())
            .map_err(std::io::Error::other)?;
        std::fs::write(db_path.join("metadata.ron"), ron)?;

        Ok(Self {
            path: db_path,
            metadata,
            tables: HashMap::new(),
        })
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<Column>) -> io::Result<()> {
        let table_meta = TableMetadata {
            name: name.to_string(),
            columns,
            row_count: 0,
            page_size: 4096,
        };

        // Save table schema
        let tables_dir = self.path.join("tables");
        let meta_path = tables_dir.join(format!("{}.meta.ron", name));
        let ron = ron::ser::to_string_pretty(&table_meta, Default::default())
            .map_err(std::io::Error::other)?;
        std::fs::write(meta_path, ron)?;

        // Save data to binary file
        let data_path = tables_dir.join(format!("{}.data.bin", name));
        std::fs::File::create(data_path)?;

        // Update metadata tables
        self.metadata.tables.push(name.to_string());
        self.save_metadata()?;

        Ok(())
    }

    fn save_metadata(&self) -> io::Result<()> {
        let ron = ron::ser::to_string_pretty(&self.metadata, Default::default())
            .map_err(std::io::Error::other)?;
        std::fs::write(self.path.join("metadata.ron"), ron)?;
        Ok(())
    }
}

/// Structure of a table
struct Table {
    metadata: TableMetadata,
    data_file: std::fs::File,
}

impl Table {
    /// Inserts a single row in current instance of Table
    fn insert_row(&mut self, row: Vec<Value>) -> io::Result<()> {
        let config = config::standard();
        let encoded = bincode::encode_to_vec(&row, config).map_err(std::io::Error::other)?;

        // Aller à la fin du fichier
        self.data_file.seek(SeekFrom::End(0))?;

        // Écrire [longueur][données]
        self.data_file
            .write_all(&(encoded.len() as u32).to_le_bytes())?;
        self.data_file.write_all(&encoded)?;
        self.data_file.flush()?;

        self.metadata.row_count += 1;
        Ok(())
    }

    /// Reads all rows from given instance
    fn read_all_rows(&mut self) -> io::Result<Vec<Vec<Value>>> {
        self.data_file.seek(SeekFrom::Start(0))?;
        let mut rows = Vec::new();
        let config = config::standard();

        loop {
            let mut len_bytes = [0u8; 4];
            match self.data_file.read_exact(&mut len_bytes) {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let len = u32::from_le_bytes(len_bytes) as usize;

            let mut data = vec![0u8; len];
            self.data_file.read_exact(&mut data)?;

            let (row, _): (Vec<Value>, usize) =
                bincode::decode_from_slice(&data, config).map_err(std::io::Error::other)?;

            rows.push(row);
        }

        Ok(rows)
    }
}

#[derive(Serialize, Deserialize, Debug, Encode, Decode)]
enum Value {
    Integer(i64),
    Text(String),
    Real(f64),
    Null,
}
