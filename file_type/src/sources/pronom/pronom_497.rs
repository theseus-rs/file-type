use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_497: FileFormat = FileFormat {
    id: 497,
    source_type: SourceType::Pronom,
    name: "Lotus Approach View File",
    extensions: &["apt"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
