use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135299: FileType = FileType {
    file_format: &FileFormat {
        id: 135_299,
        source_type: SourceType::Wikidata,
        name: "Dynamic Adaptive Streaming over HTTP",
        extensions: &["mpd"],
        media_types: &["application/dash+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
