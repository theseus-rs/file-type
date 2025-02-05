use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2655: FileFormat = FileFormat {
    id: 2_655,
    source_type: SourceType::Pronom,
    name: "B Source Code File",
    extensions: &["b"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
