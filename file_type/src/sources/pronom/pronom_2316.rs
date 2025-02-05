use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2316: FileFormat = FileFormat {
    id: 2_316,
    source_type: SourceType::Pronom,
    name: "NTI JewelCase Maker",
    extensions: &["jwc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
