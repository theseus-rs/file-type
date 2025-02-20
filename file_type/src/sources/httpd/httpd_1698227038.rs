use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1698227038: FileType = FileType {
    file_format: &FileFormat {
        id: 1_698_227_038,
        source_type: SourceType::Httpd,
        name: "ms excel addin macroenabled 12",
        extensions: &["xlam"],
        media_types: &["application/vnd.ms-excel.addin.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
