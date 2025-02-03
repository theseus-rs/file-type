use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2000: FileFormat = FileFormat {
    id: 2_000,
    source_type: SourceType::Pronom,
    name: "Adobe SWC Package",
    extensions: &["swc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
