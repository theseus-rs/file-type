use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_269: FileFormat = FileFormat {
    id: 269,
    source_type: SourceType::Pronom,
    name: "NeXt Sound",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
