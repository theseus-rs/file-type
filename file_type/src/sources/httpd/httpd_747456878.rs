use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_747456878: FileType = FileType {
    file_format: &FileFormat {
        id: 747_456_878,
        source_type: SourceType::Httpd,
        name: "macports portpkg",
        extensions: &["portpkg"],
        media_types: &["application/vnd.macports.portpkg"],
        signatures: &[],
        related_formats: &[],
    },
};
