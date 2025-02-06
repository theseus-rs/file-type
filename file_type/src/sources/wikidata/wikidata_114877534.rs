use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877534: FileFormat = FileFormat {
    id: 114_877_534,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Card file",
    extensions: &["sph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
