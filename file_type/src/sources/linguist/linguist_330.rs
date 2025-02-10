use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_330: FileType = FileType {
    file_format: &FileFormat {
        id: 330,
        source_type: SourceType::Linguist,
        name: "SMT",
        extensions: &["smt", "smt2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
