use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113161974: FileFormat = FileFormat {
    id: 113_161_974,
    puid: "wikidata/113161974",
    name: "Act! database file",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
