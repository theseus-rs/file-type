use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967222: FileFormat = FileFormat {
    id: 27_967_222,
    puid: "wikidata/27967222",
    name: "Soundtrakker 128 module",
    extensions: &["128"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
