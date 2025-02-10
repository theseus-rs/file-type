use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2733296093: FileType = FileType {
    file_format: &FileFormat {
        id: 2_733_296_093,
        source_type: SourceType::Iana,
        name: "vnd.nimn",
        extensions: &[],
        media_types: &["application/vnd.nimn"],
        signatures: &[],
        related_formats: &[],
    },
};
