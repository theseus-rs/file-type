use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_436: FileFormat = FileFormat {
    id: 436,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Revisable Form Text File",
    extensions: &["rft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
