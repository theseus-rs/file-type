use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877461: FileFormat = FileFormat {
    id: 114_877_461,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Mini Album file",
    extensions: &["sma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
