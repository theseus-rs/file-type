use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123378531: FileType = FileType {
    file_format: &FileFormat {
        id: 123_378_531,
        source_type: SourceType::Wikidata,
        name: "Curve library",
        extensions: &["cvl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
