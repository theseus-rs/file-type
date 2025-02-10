use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_910732762: FileType = FileType {
    file_format: &FileFormat {
        id: 910_732_762,
        source_type: SourceType::Httpd,
        name: "kidspiration",
        extensions: &["kia"],
        media_types: &["application/vnd.kidspiration"],
        signatures: &[],
        related_formats: &[],
    },
};
