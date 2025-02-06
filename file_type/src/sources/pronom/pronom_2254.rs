use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2254: FileFormat = FileFormat {
    id: 2_254,
    source_type: SourceType::Pronom,
    name: "Minitab Project",
    extensions: &["mpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
