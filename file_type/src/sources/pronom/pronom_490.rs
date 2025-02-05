use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_490: FileFormat = FileFormat {
    id: 490,
    source_type: SourceType::Pronom,
    name: "IntelliDraw Vector Graphics",
    extensions: &["idw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
