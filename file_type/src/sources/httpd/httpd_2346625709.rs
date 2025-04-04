use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2346625709: FileType = FileType {
    file_format: &FileFormat {
        id: 2_346_625_709,
        source_type: SourceType::Httpd,
        name: "geonext",
        extensions: &["gxt"],
        media_types: &["application/vnd.geonext"],
        signatures: &[],
        related_formats: &[],
    },
};
