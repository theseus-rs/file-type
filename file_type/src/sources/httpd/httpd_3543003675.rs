use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3543003675: FileType = FileType {
    file_format: &FileFormat {
        id: 3_543_003_675,
        source_type: SourceType::Httpd,
        name: "cache manifest",
        extensions: &["appcache"],
        media_types: &["text/cache-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
