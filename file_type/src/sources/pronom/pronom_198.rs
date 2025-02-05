use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_198: FileFormat = FileFormat {
    id: 198,
    source_type: SourceType::Pronom,
    name: "Active Server Page",
    extensions: &["asp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
