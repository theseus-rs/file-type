use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1968309032: FileType = FileType {
    file_format: &FileFormat {
        id: 1_968_309_032,
        source_type: SourceType::Httpd,
        name: "recordare musicxml",
        extensions: &["mxl"],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
