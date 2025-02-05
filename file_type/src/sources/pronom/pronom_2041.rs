use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2041: FileFormat = FileFormat {
    id: 2_041,
    source_type: SourceType::Pronom,
    name: "Sibelius Sound Set Definition",
    extensions: &["set"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
