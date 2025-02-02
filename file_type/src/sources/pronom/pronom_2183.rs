use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2183: FileFormat = FileFormat {
    id: 2_183,
    source_type: SourceType::Pronom,
    name: "Debug File",
    extensions: &["dbg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
