use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2185967478: FileType = FileType {
    file_format: &FileFormat {
        id: 2_185_967_478,
        source_type: SourceType::Iana,
        name: "teep+cbor",
        extensions: &[],
        media_types: &["application/teep+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
