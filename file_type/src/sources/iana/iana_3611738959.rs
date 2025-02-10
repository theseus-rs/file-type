use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3611738959: FileType = FileType {
    file_format: &FileFormat {
        id: 3_611_738_959,
        source_type: SourceType::Iana,
        name: "eat-bun+cbor",
        extensions: &[],
        media_types: &["application/eat-bun+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
