use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125808516: FileType = FileType {
    file_format: &FileFormat {
        id: 125_808_516,
        source_type: SourceType::Wikidata,
        name: "Mnemosyne Flash-card Collection",
        extensions: &["mem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
