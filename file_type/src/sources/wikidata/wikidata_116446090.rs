use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116446090: FileFormat = FileFormat {
    id: 116_446_090,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Web JukeBox File",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
