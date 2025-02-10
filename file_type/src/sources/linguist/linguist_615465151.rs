use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_615465151: FileType = FileType {
    file_format: &FileFormat {
        id: 615_465_151,
        source_type: SourceType::Linguist,
        name: "Caddyfile",
        extensions: &["caddyfile"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
