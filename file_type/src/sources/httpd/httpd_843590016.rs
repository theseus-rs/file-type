use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_843590016: FileType = FileType {
    file_format: &FileFormat {
        id: 843_590_016,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument wordprocessingml document",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
