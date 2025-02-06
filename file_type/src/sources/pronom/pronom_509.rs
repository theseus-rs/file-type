use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_509: FileFormat = FileFormat {
    id: 509,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Document",
    extensions: &["bps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
