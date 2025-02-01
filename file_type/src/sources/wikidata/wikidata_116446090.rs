use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116446090: FileFormat = FileFormat {
    id: 116_446_090,
    puid: "wikidata/116446090",
    name: "CoffeeCup Web JukeBox File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
