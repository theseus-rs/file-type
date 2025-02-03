use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_198: FileFormat = FileFormat {
    id: 198,
    source_type: SourceType::Pronom,
    name: "Active Server Page",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
