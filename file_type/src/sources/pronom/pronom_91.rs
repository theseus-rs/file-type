use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_91: FileFormat = FileFormat {
    id: 91,
    source_type: SourceType::Pronom,
    name: "Ventura Publisher Vector Graphics",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
