use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_330: FileType = FileType {
    file_format: &FileFormat {
        id: 330,
        source_type: SourceType::Linguist,
        name: "SMT",
        extensions: &["smt", "smt2", "z3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
