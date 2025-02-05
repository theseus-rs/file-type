use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2191: FileFormat = FileFormat {
    id: 2_191,
    source_type: SourceType::Pronom,
    name: "OmniPage Document",
    extensions: &["opd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
