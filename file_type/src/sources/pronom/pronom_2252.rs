use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2252: FileFormat = FileFormat {
    id: 2_252,
    source_type: SourceType::Pronom,
    name: "Minitab Project",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
