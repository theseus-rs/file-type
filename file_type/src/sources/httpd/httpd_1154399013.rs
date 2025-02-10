use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1154399013: FileType = FileType {
    file_format: &FileFormat {
        id: 1_154_399_013,
        source_type: SourceType::Httpd,
        name: "onenote",
        extensions: &["onetoc", "onetoc2", "onetmp", "onepkg"],
        media_types: &["application/onenote"],
        signatures: &[],
        related_formats: &[],
    },
};
