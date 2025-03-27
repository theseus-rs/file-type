use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122212378: FileType = FileType {
    file_format: &FileFormat {
        id: 122_212_378,
        source_type: SourceType::Wikidata,
        name: "App",
        extensions: &["app", "hap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
