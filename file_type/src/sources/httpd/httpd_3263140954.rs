use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3263140954: FileType = FileType {
    file_format: &FileFormat {
        id: 3_263_140_954,
        source_type: SourceType::Httpd,
        name: "ms excel sheet macroenabled 12",
        extensions: &["xlsm"],
        media_types: &["application/vnd.ms-excel.sheet.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
