use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1359614180: FileType = FileType {
    file_format: &FileFormat {
        id: 1_359_614_180,
        source_type: SourceType::Httpd,
        name: "gdl",
        extensions: &["gdl"],
        media_types: &["model/vnd.gdl"],
        signatures: &[],
        related_formats: &[],
    },
};
