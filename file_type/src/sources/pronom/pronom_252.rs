use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_252: FileFormat = FileFormat {
    id: 252,
    source_type: SourceType::Pronom,
    name: "Microsoft Visual Modeller Petal file (ASCII)",
    extensions: &["ptl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
