use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116445963: FileFormat = FileFormat {
    id: 116_445_963,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Web Video Player File",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
