use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850367: FileFormat = FileFormat {
    id: 105_850_367,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM COMLOCK encrypted (v0.10)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
