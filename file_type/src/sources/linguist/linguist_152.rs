use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_152: FileType = FileType {
    file_format: &FileFormat {
        id: 152,
        source_type: SourceType::Linguist,
        name: "HTTP",
        extensions: &["http"],
        media_types: &["message/http"],
        signatures: &[],
        related_formats: &[],
    },
};
