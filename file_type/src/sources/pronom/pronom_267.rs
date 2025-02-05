use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_267: FileFormat = FileFormat {
    id: 267,
    source_type: SourceType::Pronom,
    name: "IRIS Graphics",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
