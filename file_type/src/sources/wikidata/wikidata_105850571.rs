use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850571: FileFormat = FileFormat {
    id: 105_850_571,
    puid: "wikidata/105850571",
    name: "16bit DOS COM MASK encrypted (v2.x)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
