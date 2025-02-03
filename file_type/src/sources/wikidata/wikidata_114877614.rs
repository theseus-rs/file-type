use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877614: FileFormat = FileFormat {
    id: 114_877_614,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe T-Shirt file",
    extensions: &["ste"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
