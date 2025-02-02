use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_490: FileFormat = FileFormat {
    id: 490,
    source_type: SourceType::Pronom,
    name: "IntelliDraw Vector Graphics",
    extensions: &["idw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
