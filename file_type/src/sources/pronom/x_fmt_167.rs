use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_167: FileFormat = FileFormat {
    id: 239,
    puid: "x-fmt/167",
    name: "Adobe PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
