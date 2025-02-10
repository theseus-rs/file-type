use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_808640727: FileType = FileType {
    file_format: &FileFormat {
        id: 808_640_727,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder7-s",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder7-s"],
        signatures: &[],
        related_formats: &[],
    },
};
