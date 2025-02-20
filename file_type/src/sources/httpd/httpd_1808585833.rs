use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1808585833: FileType = FileType {
    file_format: &FileFormat {
        id: 1_808_585_833,
        source_type: SourceType::Httpd,
        name: "ms powerpoint addin macroenabled 12",
        extensions: &["ppam"],
        media_types: &["application/vnd.ms-powerpoint.addin.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
