use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_539: FileFormat = FileFormat {
    id: 539,
    source_type: SourceType::Pronom,
    name: "Vista Pro Graphics",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
