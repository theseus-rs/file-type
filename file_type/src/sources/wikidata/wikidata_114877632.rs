use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877632: FileFormat = FileFormat {
    id: 114_877_632,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Slide Show file",
    extensions: &["sss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
