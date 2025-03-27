use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62617958: FileType = FileType {
    file_format: &FileFormat {
        id: 62_617_958,
        source_type: SourceType::Wikidata,
        name: "WebP",
        extensions: &["webp"],
        media_types: &["image/webp"],
        signatures: &[],
        related_formats: &[],
    },
};
