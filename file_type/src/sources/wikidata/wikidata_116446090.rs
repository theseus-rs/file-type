use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116446090: FileFormat = FileFormat {
    id: 116_446_090,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Web JukeBox File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
