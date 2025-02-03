use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_219: FileFormat = FileFormat {
    id: 219,
    source_type: SourceType::Pronom,
    name: "Ventura Publisher",
    extensions: &["gen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
