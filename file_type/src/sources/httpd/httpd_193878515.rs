use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_193878515: FileType = FileType {
    file_format: &FileFormat {
        id: 193_878_515,
        source_type: SourceType::Httpd,
        name: "sdp",
        extensions: &["sdp"],
        media_types: &["application/sdp"],
        signatures: &[],
        related_formats: &[],
    },
};
