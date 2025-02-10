use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2767865714: FileType = FileType {
    file_format: &FileFormat {
        id: 2_767_865_714,
        source_type: SourceType::Httpd,
        name: "hp jlyt",
        extensions: &["jlt"],
        media_types: &["application/vnd.hp-jlyt"],
        signatures: &[],
        related_formats: &[],
    },
};
