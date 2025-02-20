use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110535991: FileType = FileType {
    file_format: &FileFormat {
        id: 110_535_991,
        source_type: SourceType::Wikidata,
        name: "Movie Magic Screenwriter backup document",
        extensions: &["bk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
