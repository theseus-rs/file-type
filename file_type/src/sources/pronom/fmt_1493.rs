use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1493: FileFormat = FileFormat {
    id: 2_316,
    puid: "fmt/1493",
    name: "NTI JewelCase Maker",
    extensions: &["jwc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
