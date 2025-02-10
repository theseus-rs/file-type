use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_84: FileType = FileType {
    file_format: &FileFormat {
        id: 84,
        source_type: SourceType::Linguist,
        name: "DNS Zone",
        extensions: &["arpa", "zone"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
