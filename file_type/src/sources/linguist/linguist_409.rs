use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_409: FileType = FileType {
    file_format: &FileFormat {
        id: 409,
        source_type: SourceType::Linguist,
        name: "Yacc",
        extensions: &["y", "yacc", "yy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
