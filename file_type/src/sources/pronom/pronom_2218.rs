use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2218: FileFormat = FileFormat {
    id: 2_218,
    source_type: SourceType::Pronom,
    name: "Ichitaro Document",
    extensions: &["jtd", "jtt", "$td"],
    media_types: &["application/x-js-taro"],
    internal_signatures: &[],
    related_formats: &[],
};
