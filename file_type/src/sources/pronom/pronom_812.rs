use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_812: FileFormat = FileFormat {
    id: 812,
    source_type: SourceType::Pronom,
    name: "Generic Library File",
    extensions: &["lib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
