use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1298535354: FileType = FileType {
    file_format: &FileFormat {
        id: 1_298_535_354,
        source_type: SourceType::Httpd,
        name: "jcp javame midlet rms",
        extensions: &["rms"],
        media_types: &["application/vnd.jcp.javame.midlet-rms"],
        signatures: &[],
        related_formats: &[],
    },
};
