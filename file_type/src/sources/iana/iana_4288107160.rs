use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4288107160: FileType = FileType {
    file_format: &FileFormat {
        id: 4_288_107_160,
        source_type: SourceType::Iana,
        name: "cbor",
        extensions: &[],
        media_types: &["application/cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
