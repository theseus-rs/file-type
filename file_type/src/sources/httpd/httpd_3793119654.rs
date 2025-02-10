use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3793119654: FileType = FileType {
    file_format: &FileFormat {
        id: 3_793_119_654,
        source_type: SourceType::Httpd,
        name: "ms officetheme",
        extensions: &["thmx"],
        media_types: &["application/vnd.ms-officetheme"],
        signatures: &[],
        related_formats: &[],
    },
};
