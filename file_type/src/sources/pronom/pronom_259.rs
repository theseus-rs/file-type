use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_259: FileFormat = FileFormat {
    id: 259,
    source_type: SourceType::Pronom,
    name: "Silicon Graphics RGB File",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
