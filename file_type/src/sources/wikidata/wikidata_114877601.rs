use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877601: FileFormat = FileFormat {
    id: 114_877_601,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Sticker file",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
