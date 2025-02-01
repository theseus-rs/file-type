use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850251: FileFormat = FileFormat {
    id: 105_850_251,
    puid: "wikidata/105850251",
    name: "16bit DOS COM The WiZ Cryptor encrypted (v1.00a)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
