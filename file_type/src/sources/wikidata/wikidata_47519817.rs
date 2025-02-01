use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519817: FileFormat = FileFormat {
    id: 47_519_817,
    puid: "wikidata/47519817",
    name: "Serif PagePlus Publication file format, version 5",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
