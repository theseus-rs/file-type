use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_539: FileFormat = FileFormat {
    id: 539,
    source_type: SourceType::Pronom,
    name: "Vista Pro Graphics",
    extensions: &["dem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
