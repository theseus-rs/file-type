use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116445963: FileFormat = FileFormat {
    id: 116_445_963,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Web Video Player File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
