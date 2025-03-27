use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45432: FileType = FileType {
    file_format: &FileFormat {
        id: 45_432,
        source_type: SourceType::Wikidata,
        name: "RSS",
        extensions: &["rss", "xml"],
        media_types: &["application/rss+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
