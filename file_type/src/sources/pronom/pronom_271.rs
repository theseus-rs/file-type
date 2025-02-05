use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_271: FileFormat = FileFormat {
    id: 271,
    source_type: SourceType::Pronom,
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
