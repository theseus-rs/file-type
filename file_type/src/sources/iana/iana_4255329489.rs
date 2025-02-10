use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4255329489: FileType = FileType {
    file_format: &FileFormat {
        id: 4_255_329_489,
        source_type: SourceType::Iana,
        name: "ace-groupcomm+cbor",
        extensions: &[],
        media_types: &["application/ace-groupcomm+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
