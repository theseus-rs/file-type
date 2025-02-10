use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2652912783: FileType = FileType {
    file_format: &FileFormat {
        id: 2_652_912_783,
        source_type: SourceType::Httpd,
        name: "jsonml json",
        extensions: &["jsonml"],
        media_types: &["application/jsonml+json"],
        signatures: &[],
        related_formats: &[],
    },
};
