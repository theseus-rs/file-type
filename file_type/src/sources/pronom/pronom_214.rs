use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_214: FileFormat = FileFormat {
    id: 214,
    source_type: SourceType::Pronom,
    name: "Digital Video",
    extensions: &["dv"],
    media_types: &["video/dv"],
    internal_signatures: &[],
    related_formats: &[],
};
