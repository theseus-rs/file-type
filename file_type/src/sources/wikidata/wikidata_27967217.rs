use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967217: FileFormat = FileFormat {
    id: 27_967_217,
    puid: "wikidata/27967217",
    name: "Scream Tracker Music Interface Kit module",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
