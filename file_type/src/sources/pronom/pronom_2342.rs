use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2342: FileFormat = FileFormat {
    id: 2_342,
    source_type: SourceType::Pronom,
    name: "Serif PhotoPlus Image",
    extensions: &["spp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
