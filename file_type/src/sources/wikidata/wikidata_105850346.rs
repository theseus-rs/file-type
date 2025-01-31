use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850346: FileFormat = FileFormat {
    id: 105_850_346,
    puid: "wikidata/105850346",
    name: "16bit DOS COM mCrypt encrypted (v0.1b)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
