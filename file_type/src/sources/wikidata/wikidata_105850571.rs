use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850571: FileFormat = FileFormat {
    id: 105_850_571,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM MASK encrypted (v2.x)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
