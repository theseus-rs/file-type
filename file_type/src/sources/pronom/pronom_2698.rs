use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2698: FileFormat = FileFormat {
    id: 2_698,
    source_type: SourceType::Pronom,
    name: "Fountain Markup Language File",
    extensions: &["spmd", "fountain"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
