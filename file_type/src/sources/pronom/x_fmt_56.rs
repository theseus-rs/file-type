use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_56: FileFormat = FileFormat {
    id: 90,
    puid: "x-fmt/56",
    name: "Kodak FlashPix Image",
    extensions: &["fpx"],
    media_types: &["image/vnd.fpx"],
    internal_signatures: &[],
    related_formats: &[],
};
