use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_261: FileType = FileType {
    file_format: &FileFormat {
        id: 261,
        source_type: SourceType::Linguist,
        name: "Opa",
        extensions: &["opa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
