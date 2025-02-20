use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1490533465: FileType = FileType {
    file_format: &FileFormat {
        id: 1_490_533_465,
        source_type: SourceType::Iana,
        name: "sensml+cbor",
        extensions: &[],
        media_types: &["application/sensml+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
