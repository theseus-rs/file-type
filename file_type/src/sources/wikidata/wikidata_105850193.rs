use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850193: FileFormat = FileFormat {
    id: 105_850_193,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM CC286 encrypted (v2.1)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
