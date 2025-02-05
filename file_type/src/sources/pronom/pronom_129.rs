use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_129: FileFormat = FileFormat {
    id: 129,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
