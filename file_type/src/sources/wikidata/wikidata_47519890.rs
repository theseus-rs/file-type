use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519890: FileFormat = FileFormat {
    id: 47_519_890,
    puid: "wikidata/47519890",
    name: "Serif PagePlus Publication file format, version 9",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
