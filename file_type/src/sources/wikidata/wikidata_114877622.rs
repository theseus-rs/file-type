use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877622: FileFormat = FileFormat {
    id: 114_877_622,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Family Tree file",
    extensions: &["sft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
