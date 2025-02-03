use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877534: FileFormat = FileFormat {
    id: 114_877_534,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Card file",
    extensions: &["sph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
