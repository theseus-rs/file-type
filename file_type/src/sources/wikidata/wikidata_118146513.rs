use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118146513: FileFormat = FileFormat {
    id: 118_146_513,
    puid: "wikidata/118146513",
    name: "Coaxial Cable File",
    extensions: &["tl7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
