use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116523877: FileFormat = FileFormat {
    id: 116_523_877,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup CD Info File",
    extensions: &["lav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
