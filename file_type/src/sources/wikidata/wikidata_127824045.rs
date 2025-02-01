use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127824045: FileFormat = FileFormat {
    id: 127_824_045,
    puid: "wikidata/127824045",
    name: "Cinema DTS Audio file format",
    extensions: &["apx", "aud", "aue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
