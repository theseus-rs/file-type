use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_136: FileFormat = FileFormat {
    id: 136,
    source_type: SourceType::Pronom,
    name: "Freelance File",
    extensions: &["pre"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
