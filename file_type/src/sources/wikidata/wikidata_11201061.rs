use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11201061: FileType = FileType {
    file_format: &FileFormat {
        id: 11_201_061,
        source_type: SourceType::Wikidata,
        name: "animated GIF",
        extensions: &["gif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
