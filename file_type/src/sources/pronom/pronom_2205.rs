use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2205: FileFormat = FileFormat {
    id: 2_205,
    source_type: SourceType::Pronom,
    name: "Muvee autoProducer Project File",
    extensions: &["mvex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
