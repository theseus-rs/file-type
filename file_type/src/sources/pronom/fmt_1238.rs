use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1238: FileFormat = FileFormat {
    id: 2_056,
    puid: "fmt/1238",
    name: "Band Interleaved By Line (BIL) Image Encoding",
    extensions: &["bil"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
