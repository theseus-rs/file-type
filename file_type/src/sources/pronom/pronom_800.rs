use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_800: FileFormat = FileFormat {
    id: 800,
    source_type: SourceType::Pronom,
    name: "Batch file (executable)",
    extensions: &["bat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
