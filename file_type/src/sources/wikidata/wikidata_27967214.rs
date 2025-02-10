use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967214: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_214,
        source_type: SourceType::Wikidata,
        name: "SBStudio module",
        extensions: &["pac", "son", "sou"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
