use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2150627582: FileType = FileType {
    file_format: &FileFormat {
        id: 2_150_627_582,
        source_type: SourceType::Httpd,
        name: "resource lists xml",
        extensions: &["rl"],
        media_types: &["application/resource-lists+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
