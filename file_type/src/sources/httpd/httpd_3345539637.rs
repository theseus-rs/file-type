use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3345539637: FileType = FileType {
    file_format: &FileFormat {
        id: 3_345_539_637,
        source_type: SourceType::Httpd,
        name: "marc",
        extensions: &["mrc"],
        media_types: &["application/marc"],
        signatures: &[],
        related_formats: &[],
    },
};
