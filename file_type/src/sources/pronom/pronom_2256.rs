use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2256: FileFormat = FileFormat {
    id: 2_256,
    source_type: SourceType::Pronom,
    name: "Minitab Project",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
