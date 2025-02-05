use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_91: FileFormat = FileFormat {
    id: 91,
    source_type: SourceType::Pronom,
    name: "Ventura Publisher Vector Graphics",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
