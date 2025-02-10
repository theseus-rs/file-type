use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3296089372: FileType = FileType {
    file_format: &FileFormat {
        id: 3_296_089_372,
        source_type: SourceType::Httpd,
        name: "curl pcurl",
        extensions: &["pcurl"],
        media_types: &["application/vnd.curl.pcurl"],
        signatures: &[],
        related_formats: &[],
    },
};
