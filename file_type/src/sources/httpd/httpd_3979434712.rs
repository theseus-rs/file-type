use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3979434712: FileType = FileType {
    file_format: &FileFormat {
        id: 3_979_434_712,
        source_type: SourceType::Httpd,
        name: "heif",
        extensions: &["heif"],
        media_types: &["image/heif"],
        signatures: &[],
        related_formats: &[],
    },
};
