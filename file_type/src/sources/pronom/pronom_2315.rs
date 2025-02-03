use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2315: FileFormat = FileFormat {
    id: 2_315,
    source_type: SourceType::Pronom,
    name: "Harvard Graphics Presentation",
    extensions: &["pr4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
