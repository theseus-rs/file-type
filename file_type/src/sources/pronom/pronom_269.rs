use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_269: FileFormat = FileFormat {
    id: 269,
    source_type: SourceType::Pronom,
    name: "NeXt Sound",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
