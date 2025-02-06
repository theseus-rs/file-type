use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2580: FileFormat = FileFormat {
    id: 2_580,
    source_type: SourceType::Pronom,
    name: "C/C++ Header File",
    extensions: &["h", "hpp", "hxx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
