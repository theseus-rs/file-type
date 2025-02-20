use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2837889784: FileType = FileType {
    file_format: &FileFormat {
        id: 2_837_889_784,
        source_type: SourceType::Httpd,
        name: "djvu",
        extensions: &["djvu", "djv"],
        media_types: &["image/vnd.djvu"],
        signatures: &[],
        related_formats: &[],
    },
};
