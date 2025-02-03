use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_362: FileFormat = FileFormat {
    id: 362,
    source_type: SourceType::Pronom,
    name: "Microsoft Project",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
