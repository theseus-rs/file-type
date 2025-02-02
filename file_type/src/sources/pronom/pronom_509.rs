use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_509: FileFormat = FileFormat {
    id: 509,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Document",
    extensions: &["bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
