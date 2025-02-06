use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2218: FileFormat = FileFormat {
    id: 2_218,
    source_type: SourceType::Pronom,
    name: "Ichitaro Document",
    extensions: &["jtd", "jtt", "$td"],
    media_types: &["application/x-js-taro"],
    signatures: &[],
    related_formats: &[],
};
