use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262857: FileFormat = FileFormat {
    id: 111_262_857,
    puid: "wikidata/111262857",
    name: "G.711 A-law european telephony format",
    extensions: &["alaw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
