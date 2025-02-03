use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_436: FileFormat = FileFormat {
    id: 436,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Revisable Form Text File",
    extensions: &["rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
