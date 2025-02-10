use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2677900451: FileType = FileType {
    file_format: &FileFormat {
        id: 2_677_900_451,
        source_type: SourceType::Httpd,
        name: "acucorp",
        extensions: &["atc", "acutc"],
        media_types: &["application/vnd.acucorp"],
        signatures: &[],
        related_formats: &[],
    },
};
