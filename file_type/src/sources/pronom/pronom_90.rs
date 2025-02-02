use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_90: FileFormat = FileFormat {
    id: 90,
    source_type: SourceType::Pronom,
    name: "Kodak FlashPix Image",
    extensions: &["fpx"],
    media_types: &["image/vnd.fpx"],
    internal_signatures: &[],
    related_formats: &[],
};
