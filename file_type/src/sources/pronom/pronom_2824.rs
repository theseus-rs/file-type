use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2824: FileFormat = FileFormat {
    id: 2_824,
    source_type: SourceType::Pronom,
    name: "Melco OFM Project",
    extensions: &["ofm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
