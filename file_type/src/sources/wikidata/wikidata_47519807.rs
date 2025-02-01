use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519807: FileFormat = FileFormat {
    id: 47_519_807,
    puid: "wikidata/47519807",
    name: "Serif PagePlus Publication file format, version 4",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
