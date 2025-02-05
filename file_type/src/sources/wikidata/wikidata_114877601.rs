use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877601: FileFormat = FileFormat {
    id: 114_877_601,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Sticker file",
    extensions: &["sb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
