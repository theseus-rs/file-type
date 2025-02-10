use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_783851439: FileType = FileType {
    file_format: &FileFormat {
        id: 783_851_439,
        source_type: SourceType::Iana,
        name: "uri-list",
        extensions: &[],
        media_types: &["text/uri-list"],
        signatures: &[],
        related_formats: &[],
    },
};
