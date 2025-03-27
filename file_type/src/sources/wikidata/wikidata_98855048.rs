use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_98855048: FileType = FileType {
    file_format: &FileFormat {
        id: 98_855_048,
        source_type: SourceType::Wikidata,
        name: "Q98855048",
        extensions: &["json", "lot"],
        media_types: &["video/lottie+json"],
        signatures: &[],
        related_formats: &[],
    },
};
