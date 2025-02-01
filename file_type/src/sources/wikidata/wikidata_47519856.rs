use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519856: FileFormat = FileFormat {
    id: 47_519_856,
    puid: "wikidata/47519856",
    name: "Serif PagePlus Publication file format, version 8",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
