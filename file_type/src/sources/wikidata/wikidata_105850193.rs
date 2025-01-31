use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850193: FileFormat = FileFormat {
    id: 105_850_193,
    puid: "wikidata/105850193",
    name: "16bit DOS COM CC286 encrypted (v2.1)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
