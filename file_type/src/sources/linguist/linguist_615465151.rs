use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
