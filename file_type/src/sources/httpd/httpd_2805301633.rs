use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2805301633: FileType = FileType {
    file_format: &FileFormat {
        id: 2_805_301_633,
        source_type: SourceType::Httpd,
        name: "curl mcurl",
        extensions: &["mcurl"],
        media_types: &["text/vnd.curl.mcurl"],
        signatures: &[],
        related_formats: &[],
    },
};
