use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1241738: FileType = FileType {
    file_format: &FileFormat {
        id: 1_241_738,
        source_type: SourceType::Wikidata,
        name: "M3U",
        extensions: &["m3u8"],
        media_types: &["audio/x-mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
