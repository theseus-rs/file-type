use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114888526: FileFormat = FileFormat {
    id: 114_888_526,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Craft file",
    extensions: &["sra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
