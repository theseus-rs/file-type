use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113621586: FileType = FileType {
    file_format: &FileFormat {
        id: 113_621_586,
        source_type: SourceType::Wikidata,
        name: "LoadRunner Raw Results",
        extensions: &["lrr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
