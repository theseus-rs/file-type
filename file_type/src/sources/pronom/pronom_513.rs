use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_513: FileFormat = FileFormat {
    id: 513,
    source_type: SourceType::Pronom,
    name: "Nota Bene Text File",
    extensions: &["nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
