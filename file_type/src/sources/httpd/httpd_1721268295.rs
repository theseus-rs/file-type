use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1721268295: FileType = FileType {
    file_format: &FileFormat {
        id: 1_721_268_295,
        source_type: SourceType::Httpd,
        name: "metalink4 xml",
        extensions: &["meta4"],
        media_types: &["application/metalink4+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
