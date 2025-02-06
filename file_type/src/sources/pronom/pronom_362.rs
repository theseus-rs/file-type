use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_362: FileFormat = FileFormat {
    id: 362,
    source_type: SourceType::Pronom,
    name: "Microsoft Project",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
