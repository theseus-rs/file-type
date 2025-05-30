use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_864231027: FileType = FileType {
    file_format: &FileFormat {
        id: 864_231_027,
        source_type: SourceType::Iana,
        name: "vnd.ocf+cbor",
        extensions: &[],
        media_types: &["application/vnd.ocf+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
