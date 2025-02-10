use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1722242485: FileType = FileType {
    file_format: &FileFormat {
        id: 1_722_242_485,
        source_type: SourceType::Iana,
        name: "vnd.google-earth.kmz",
        extensions: &[],
        media_types: &["application/vnd.google-earth.kmz"],
        signatures: &[],
        related_formats: &[],
    },
};
