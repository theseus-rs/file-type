use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3205304064: FileType = FileType {
    file_format: &FileFormat {
        id: 3_205_304_064,
        source_type: SourceType::Iana,
        name: "senml+cbor",
        extensions: &[],
        media_types: &["application/senml+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
