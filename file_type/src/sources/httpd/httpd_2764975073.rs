use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2764975073: FileType = FileType {
    file_format: &FileFormat {
        id: 2_764_975_073,
        source_type: SourceType::Httpd,
        name: "recordare musicxml xml",
        extensions: &["musicxml"],
        media_types: &["application/vnd.recordare.musicxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
