use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114888746: FileFormat = FileFormat {
    id: 114_888_746,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Envelope file",
    extensions: &["sev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
