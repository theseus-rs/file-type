use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1648625350: FileType = FileType {
    file_format: &FileFormat {
        id: 1_648_625_350,
        source_type: SourceType::Httpd,
        name: "sailingtracker track",
        extensions: &["st"],
        media_types: &["application/vnd.sailingtracker.track"],
        signatures: &[],
        related_formats: &[],
    },
};
