use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4193206547: FileType = FileType {
    file_format: &FileFormat {
        id: 4_193_206_547,
        source_type: SourceType::Httpd,
        name: "hp hpgl",
        extensions: &["hpgl"],
        media_types: &["application/vnd.hp-hpgl"],
        signatures: &[],
        related_formats: &[],
    },
};
