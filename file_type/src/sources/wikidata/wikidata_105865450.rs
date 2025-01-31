use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865450: FileFormat = FileFormat {
    id: 105_865_450,
    puid: "wikidata/105865450",
    name: "Protocol Data Unit message data",
    extensions: &["pdu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
