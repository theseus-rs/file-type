use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_513: FileFormat = FileFormat {
    id: 513,
    source_type: SourceType::Pronom,
    name: "Nota Bene Text File",
    extensions: &["nb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
