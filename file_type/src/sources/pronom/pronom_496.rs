use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_496: FileFormat = FileFormat {
    id: 496,
    source_type: SourceType::Pronom,
    name: "Lotus Approach View File",
    extensions: &["apr"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
