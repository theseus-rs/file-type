use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85415606: FileFormat = FileFormat {
    id: 85_415_606,
    puid: "wikidata/85415606",
    name: "Sonic Scenarist Closed Caption Format",
    extensions: &["scc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
