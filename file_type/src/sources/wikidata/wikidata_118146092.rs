use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118146092: FileFormat = FileFormat {
    id: 118_146_092,
    puid: "wikidata/118146092",
    name: "Edge-coupled symmetric file",
    extensions: &["tl2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
