use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_204: FileType = FileType {
    file_format: &FileFormat {
        id: 204,
        source_type: SourceType::Linguist,
        name: "Liquid",
        extensions: &["liquid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
