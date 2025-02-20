use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_743725154: FileType = FileType {
    file_format: &FileFormat {
        id: 743_725_154,
        source_type: SourceType::Iana,
        name: "vnd.oma.lwm2m+cbor",
        extensions: &[],
        media_types: &["application/vnd.oma.lwm2m+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
