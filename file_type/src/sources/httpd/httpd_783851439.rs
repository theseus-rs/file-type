use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_783851439: FileType = FileType {
    file_format: &FileFormat {
        id: 783_851_439,
        source_type: SourceType::Httpd,
        name: "uri list",
        extensions: &["uri", "uris", "urls"],
        media_types: &["text/uri-list"],
        signatures: &[],
        related_formats: &[],
    },
};
