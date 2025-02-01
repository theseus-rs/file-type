use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100244109: FileFormat = FileFormat {
    id: 100_244_109,
    puid: "wikidata/100244109",
    name: "Student Writing Center Newsletter",
    extensions: &["nl", "nlt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
