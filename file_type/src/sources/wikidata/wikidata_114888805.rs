use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114888805: FileType = FileType {
    file_format: &FileFormat {
        id: 114_888_805,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Paper file",
        extensions: &["sdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
