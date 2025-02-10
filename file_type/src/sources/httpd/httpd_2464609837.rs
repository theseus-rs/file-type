use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2464609837: FileType = FileType {
    file_format: &FileFormat {
        id: 2_464_609_837,
        source_type: SourceType::Httpd,
        name: "rip",
        extensions: &["rip"],
        media_types: &["audio/vnd.rip"],
        signatures: &[],
        related_formats: &[],
    },
};
