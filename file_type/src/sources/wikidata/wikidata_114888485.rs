use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888485: FileFormat = FileFormat {
    id: 114_888_485,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Web Album file",
    extensions: &["swa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
