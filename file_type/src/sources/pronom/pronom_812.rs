use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_812: FileFormat = FileFormat {
    id: 812,
    source_type: SourceType::Pronom,
    name: "Generic Library File",
    extensions: &["lib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
