use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3439688362: FileType = FileType {
    file_format: &FileFormat {
        id: 3_439_688_362,
        source_type: SourceType::Iana,
        name: "ohttp-chunked-req",
        extensions: &[],
        media_types: &["message/ohttp-chunked-req"],
        signatures: &[],
        related_formats: &[],
    },
};
