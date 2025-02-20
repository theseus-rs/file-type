use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2721747258: FileType = FileType {
    file_format: &FileFormat {
        id: 2_721_747_258,
        source_type: SourceType::Httpd,
        name: "collection",
        extensions: &["ttc"],
        media_types: &["font/collection"],
        signatures: &[],
        related_formats: &[],
    },
};
