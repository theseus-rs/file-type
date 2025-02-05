use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_90: FileFormat = FileFormat {
    id: 90,
    source_type: SourceType::Pronom,
    name: "Kodak FlashPix Image",
    extensions: &["fpx"],
    media_types: &["image/vnd.fpx"],
    signatures: &[],
    related_formats: &[],
};
