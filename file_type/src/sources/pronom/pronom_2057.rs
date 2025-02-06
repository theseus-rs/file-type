use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2057: FileFormat = FileFormat {
    id: 2_057,
    source_type: SourceType::Pronom,
    name: "Band Interleaved By Pixel (BIP) Image Encoding",
    extensions: &["bip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
