use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_463: FileFormat = FileFormat {
    id: 463,
    source_type: SourceType::Pronom,
    name: "Apple Sound",
    extensions: &["afc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
