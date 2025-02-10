use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_273: FileType = FileType {
    file_format: &FileFormat {
        id: 273,
        source_type: SourceType::Linguist,
        name: "PLSQL",
        extensions: &[
            "bdy", "ddl", "fnc", "pck", "pkb", "pks", "plb", "pls", "plsql", "prc", "spc", "sql",
            "tpb", "tps", "trg", "vw",
        ],
        media_types: &["text/x-plsql"],
        signatures: &[],
        related_formats: &[],
    },
};
