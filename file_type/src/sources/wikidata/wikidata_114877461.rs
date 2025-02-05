use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877461: FileFormat = FileFormat {
    id: 114_877_461,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Mini Album file",
    extensions: &["sma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
