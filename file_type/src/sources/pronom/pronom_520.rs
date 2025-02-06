use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_520: FileFormat = FileFormat {
    id: 520,
    source_type: SourceType::Pronom,
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
