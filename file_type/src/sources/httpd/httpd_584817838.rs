use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_584817838: FileType = FileType {
    file_format: &FileFormat {
        id: 584_817_838,
        source_type: SourceType::Httpd,
        name: "docbook xml",
        extensions: &["dbk"],
        media_types: &["application/docbook+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
