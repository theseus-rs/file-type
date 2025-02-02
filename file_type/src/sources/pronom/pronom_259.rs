use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_259: FileFormat = FileFormat {
    id: 259,
    source_type: SourceType::Pronom,
    name: "Silicon Graphics RGB File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
