use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114409: FileType = FileType {
    file_format: &FileFormat {
        id: 114_409,
        source_type: SourceType::Wikidata,
        name: "Turtle",
        extensions: &["ttl"],
        media_types: &["text/turtle"],
        signatures: &[],
        related_formats: &[],
    },
};
