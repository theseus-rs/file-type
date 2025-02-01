use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114093817: FileFormat = FileFormat {
    id: 114_093_817,
    puid: "wikidata/114093817",
    name: "Media Hash List",
    extensions: &["mhl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
