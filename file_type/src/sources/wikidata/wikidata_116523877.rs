use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116523877: FileFormat = FileFormat {
    id: 116_523_877,
    puid: "wikidata/116523877",
    name: "CoffeeCup CD Info File",
    extensions: &["lav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
