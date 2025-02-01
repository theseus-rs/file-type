use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850014: FileFormat = FileFormat {
    id: 105_850_014,
    puid: "wikidata/105850014",
    name: "16bit DOS COM Crypt (Alex) encrypted (v1.0)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
