use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_136: FileFormat = FileFormat {
    id: 136,
    source_type: SourceType::Pronom,
    name: "Freelance File",
    extensions: &["pre"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
