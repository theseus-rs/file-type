use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_204: FileFormat = FileFormat {
    id: 204,
    source_type: SourceType::Pronom,
    name: "OS/2 Change Control File",
    extensions: &["cin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
