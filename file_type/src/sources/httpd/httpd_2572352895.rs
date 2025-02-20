use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2572352895: FileType = FileType {
    file_format: &FileFormat {
        id: 2_572_352_895,
        source_type: SourceType::Httpd,
        name: "freehand",
        extensions: &["fh", "fhc", "fh4", "fh5", "fh7"],
        media_types: &["image/x-freehand"],
        signatures: &[],
        related_formats: &[],
    },
};
