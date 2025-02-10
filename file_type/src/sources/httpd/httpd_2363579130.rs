use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2363579130: FileType = FileType {
    file_format: &FileFormat {
        id: 2_363_579_130,
        source_type: SourceType::Httpd,
        name: "dwg",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
