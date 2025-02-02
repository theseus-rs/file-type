use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2191: FileFormat = FileFormat {
    id: 2_191,
    source_type: SourceType::Pronom,
    name: "OmniPage Document",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
