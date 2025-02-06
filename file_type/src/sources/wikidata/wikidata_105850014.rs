use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850014: FileFormat = FileFormat {
    id: 105_850_014,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Crypt (Alex) encrypted (v1.0)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
