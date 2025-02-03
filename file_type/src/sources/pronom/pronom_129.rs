use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_129: FileFormat = FileFormat {
    id: 129,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
