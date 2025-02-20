use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2887931485: FileType = FileType {
    file_format: &FileFormat {
        id: 2_887_931_485,
        source_type: SourceType::Iana,
        name: "yang-sid+json",
        extensions: &[],
        media_types: &["application/yang-sid+json"],
        signatures: &[],
        related_formats: &[],
    },
};
