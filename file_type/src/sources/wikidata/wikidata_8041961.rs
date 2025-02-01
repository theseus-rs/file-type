use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8041961: FileFormat = FileFormat {
    id: 8_041_961,
    puid: "wikidata/8041961",
    name: "eXtensible Graph Markup and Modeling Language",
    extensions: &["XGMML"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
