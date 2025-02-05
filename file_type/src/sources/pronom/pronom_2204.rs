use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2204: FileFormat = FileFormat {
    id: 2_204,
    source_type: SourceType::Pronom,
    name: "Muvee autoProducer Project File",
    extensions: &["mve"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
