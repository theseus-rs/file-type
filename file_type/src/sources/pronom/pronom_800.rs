use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_800: FileFormat = FileFormat {
    id: 800,
    source_type: SourceType::Pronom,
    name: "Batch file (executable)",
    extensions: &["bat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
