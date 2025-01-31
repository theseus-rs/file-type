use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_90407344: FileFormat = FileFormat {
    id: 90_407_344,
    puid: "wikidata/90407344",
    name: "Strand SSF",
    extensions: &["ssf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
