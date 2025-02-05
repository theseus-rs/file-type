use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1317: FileFormat = FileFormat {
    id: 1_317,
    source_type: SourceType::Pronom,
    name: "eRuby HTML document",
    extensions: &["rhtml", "rhtm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
