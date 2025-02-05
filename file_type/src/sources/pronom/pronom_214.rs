use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_214: FileFormat = FileFormat {
    id: 214,
    source_type: SourceType::Pronom,
    name: "Digital Video",
    extensions: &["dv"],
    media_types: &["video/dv"],
    signatures: &[],
    related_formats: &[],
};
