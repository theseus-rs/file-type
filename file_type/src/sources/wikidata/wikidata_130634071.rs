use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130634071: FileType = FileType {
    file_format: &FileFormat {
        id: 130_634_071,
        source_type: SourceType::Wikidata,
        name: "RITA file format",
        extensions: &["rita"],
        media_types: &["text/rita"],
        signatures: &[],
        related_formats: &[],
    },
};
