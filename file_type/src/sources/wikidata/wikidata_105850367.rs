use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850367: FileFormat = FileFormat {
    id: 105_850_367,
    puid: "wikidata/105850367",
    name: "16bit DOS COM COMLOCK encrypted (v0.10)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
