use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3229630693: FileType = FileType {
    file_format: &FileFormat {
        id: 3_229_630_693,
        source_type: SourceType::Httpd,
        name: "lotus organizer",
        extensions: &["org"],
        media_types: &["application/vnd.lotus-organizer"],
        signatures: &[],
        related_formats: &[],
    },
};
