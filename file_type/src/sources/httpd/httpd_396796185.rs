use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_396796185: FileType = FileType {
    file_format: &FileFormat {
        id: 396_796_185,
        source_type: SourceType::Httpd,
        name: "gtar",
        extensions: &["gtar"],
        media_types: &["application/x-gtar"],
        signatures: &[],
        related_formats: &[],
    },
};
