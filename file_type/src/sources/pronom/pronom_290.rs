use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_290: FileFormat = FileFormat {
    id: 290,
    source_type: SourceType::Pronom,
    name: "SDSC Image Tool X Window Dump Format",
    extensions: &["xwd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
