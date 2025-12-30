use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_776270175: FileType = FileType {
    file_format: &FileFormat {
        id: 776_270_175,
        source_type: SourceType::Iana,
        name: "cmw+cbor",
        extensions: &[],
        media_types: &["application/cmw+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
