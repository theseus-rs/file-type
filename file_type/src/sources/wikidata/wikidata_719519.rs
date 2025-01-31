use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_719519: FileFormat = FileFormat {
    id: 719_519,
    puid: "wikidata/719519",
    name: "Forsyth–Edwards Notation",
    extensions: &["fen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
