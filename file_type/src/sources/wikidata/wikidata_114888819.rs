use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114888819: FileFormat = FileFormat {
    id: 114_888_819,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Embelishment file",
    extensions: &["seb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
