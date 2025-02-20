use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_720859680: FileType = FileType {
    file_format: &FileFormat {
        id: 720_859_680,
        source_type: SourceType::Linguist,
        name: "Ballerina",
        extensions: &["bal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
