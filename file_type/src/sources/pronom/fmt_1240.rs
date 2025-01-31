use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1240: FileFormat = FileFormat {
    id: 2_058,
    puid: "fmt/1240",
    name: "Band Sequential (BSQ) Image Encoding",
    extensions: &["bsq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
