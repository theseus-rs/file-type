use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888746: FileFormat = FileFormat {
    id: 114_888_746,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Envelope file",
    extensions: &["sev"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
