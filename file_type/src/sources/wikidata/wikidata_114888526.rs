use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888526: FileFormat = FileFormat {
    id: 114_888_526,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Craft file",
    extensions: &["sra"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
