use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3065629133: FileType = FileType {
    file_format: &FileFormat {
        id: 3_065_629_133,
        source_type: SourceType::Httpd,
        name: "cloanto rp9",
        extensions: &["rp9"],
        media_types: &["application/vnd.cloanto.rp9"],
        signatures: &[],
        related_formats: &[],
    },
};
