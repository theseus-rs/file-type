use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888819: FileFormat = FileFormat {
    id: 114_888_819,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Embelishment file",
    extensions: &["seb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
