use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3603794039: FileType = FileType {
    file_format: &FileFormat {
        id: 3_603_794_039,
        source_type: SourceType::Httpd,
        name: "font pcf",
        extensions: &["pcf"],
        media_types: &["application/x-font-pcf"],
        signatures: &[],
        related_formats: &[],
    },
};
