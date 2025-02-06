use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2183: FileFormat = FileFormat {
    id: 2_183,
    source_type: SourceType::Pronom,
    name: "Debug File",
    extensions: &["dbg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
