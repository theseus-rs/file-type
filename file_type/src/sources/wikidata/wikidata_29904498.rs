use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29904498: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_498,
        source_type: SourceType::Wikidata,
        name: "Rayshade Heightfield",
        extensions: &["hf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
