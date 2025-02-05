use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2256: FileFormat = FileFormat {
    id: 2_256,
    source_type: SourceType::Pronom,
    name: "Minitab Project",
    extensions: &["mpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
