use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1142814227: FileType = FileType {
    file_format: &FileFormat {
        id: 1_142_814_227,
        source_type: SourceType::Httpd,
        name: "crick clicker palette",
        extensions: &["clkp"],
        media_types: &["application/vnd.crick.clicker.palette"],
        signatures: &[],
        related_formats: &[],
    },
};
