use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850640: FileFormat = FileFormat {
    id: 105_850_640,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM ComProtector encrypted (v1.0)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
