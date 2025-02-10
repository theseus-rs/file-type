use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_406395330: FileType = FileType {
    file_format: &FileFormat {
        id: 406_395_330,
        source_type: SourceType::Linguist,
        name: "Jison Lex",
        extensions: &["jisonlex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
