use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79239177: FileFormat = FileFormat {
    id: 79_239_177,
    puid: "wikidata/79239177",
    name: "The Bat! Address Book",
    extensions: &["abd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
