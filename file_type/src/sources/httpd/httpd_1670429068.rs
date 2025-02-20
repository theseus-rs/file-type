use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1670429068: FileType = FileType {
    file_format: &FileFormat {
        id: 1_670_429_068,
        source_type: SourceType::Httpd,
        name: "iges",
        extensions: &["igs", "iges"],
        media_types: &["model/iges"],
        signatures: &[],
        related_formats: &[],
    },
};
