use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1239: FileFormat = FileFormat {
    id: 2_057,
    puid: "fmt/1239",
    name: "Band Interleaved By Pixel (BIP) Image Encoding",
    extensions: &["bip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
