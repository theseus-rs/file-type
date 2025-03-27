use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25378623: FileType = FileType {
    file_format: &FileFormat {
        id: 25_378_623,
        source_type: SourceType::Wikidata,
        name: "Font-File Format",
        extensions: &["fnt", "fon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
