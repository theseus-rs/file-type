use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1041591162: FileType = FileType {
    file_format: &FileFormat {
        id: 1_041_591_162,
        source_type: SourceType::Iana,
        name: "cmw+jws",
        extensions: &[],
        media_types: &["application/cmw+jws"],
        signatures: &[],
        related_formats: &[],
    },
};
