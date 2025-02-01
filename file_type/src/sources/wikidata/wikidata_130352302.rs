use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130352302: FileFormat = FileFormat {
    id: 130_352_302,
    puid: "wikidata/130352302",
    name: "Monte file",
    extensions: &["mt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
