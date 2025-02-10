use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_794273739: FileType = FileType {
    file_format: &FileFormat {
        id: 794_273_739,
        source_type: SourceType::Httpd,
        name: "curl scurl",
        extensions: &["scurl"],
        media_types: &["text/vnd.curl.scurl"],
        signatures: &[],
        related_formats: &[],
    },
};
