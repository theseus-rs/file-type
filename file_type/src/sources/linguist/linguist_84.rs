use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
