use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_210859652: FileType = FileType {
    file_format: &FileFormat {
        id: 210_859_652,
        source_type: SourceType::Httpd,
        name: "fuzzysheet",
        extensions: &["fzs"],
        media_types: &["application/vnd.fuzzysheet"],
        signatures: &[],
        related_formats: &[],
    },
};
