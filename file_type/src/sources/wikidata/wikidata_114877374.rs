use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877374: FileFormat = FileFormat {
    id: 114_877_374,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Journal file",
    extensions: &["sjd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
