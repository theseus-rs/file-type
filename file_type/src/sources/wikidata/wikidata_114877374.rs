use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877374: FileFormat = FileFormat {
    id: 114_877_374,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Journal file",
    extensions: &["sjd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
