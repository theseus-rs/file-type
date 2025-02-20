use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1400753887: FileType = FileType {
    file_format: &FileFormat {
        id: 1_400_753_887,
        source_type: SourceType::Httpd,
        name: "css",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
