use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1568461348: FileType = FileType {
    file_format: &FileFormat {
        id: 1_568_461_348,
        source_type: SourceType::Iana,
        name: "vnd.data-vision.rdz",
        extensions: &[],
        media_types: &["application/vnd.data-vision.rdz"],
        signatures: &[],
        related_formats: &[],
    },
};
