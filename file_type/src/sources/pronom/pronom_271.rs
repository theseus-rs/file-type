use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_271: FileFormat = FileFormat {
    id: 271,
    source_type: SourceType::Pronom,
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
