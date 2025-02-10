use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2734779849: FileType = FileType {
    file_format: &FileFormat {
        id: 2_734_779_849,
        source_type: SourceType::Httpd,
        name: "gtw",
        extensions: &["gtw"],
        media_types: &["model/vnd.gtw"],
        signatures: &[],
        related_formats: &[],
    },
};
