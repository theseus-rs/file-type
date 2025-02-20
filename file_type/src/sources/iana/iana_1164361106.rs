use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1164361106: FileType = FileType {
    file_format: &FileFormat {
        id: 1_164_361_106,
        source_type: SourceType::Iana,
        name: "fits",
        extensions: &[],
        media_types: &["application/fits"],
        signatures: &[],
        related_formats: &[],
    },
};
