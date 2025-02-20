use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757958: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_958,
        source_type: SourceType::Wikidata,
        name: "Hangul Word Processor Document",
        extensions: &["hwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
