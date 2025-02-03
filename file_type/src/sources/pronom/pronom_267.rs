use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_267: FileFormat = FileFormat {
    id: 267,
    source_type: SourceType::Pronom,
    name: "IRIS Graphics",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
