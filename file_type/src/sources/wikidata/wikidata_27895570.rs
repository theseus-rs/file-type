use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27895570: FileFormat = FileFormat {
    id: 27_895_570,
    puid: "wikidata/27895570",
    name: "ADX, version 5",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
