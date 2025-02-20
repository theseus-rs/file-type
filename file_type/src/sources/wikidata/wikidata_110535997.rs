use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110535997: FileType = FileType {
    file_format: &FileFormat {
        id: 110_535_997,
        source_type: SourceType::Wikidata,
        name: "Movie Magic Screenwriter timed backup document",
        extensions: &["tmb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
