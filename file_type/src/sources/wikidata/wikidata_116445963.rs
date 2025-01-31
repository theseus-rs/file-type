use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116445963: FileFormat = FileFormat {
    id: 116_445_963,
    puid: "wikidata/116445963",
    name: "CoffeeCup Web Video Player File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
