use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_401: FileFormat = FileFormat {
    id: 755,
    puid: "x-fmt/401",
    name: "StarOffice Draw",
    extensions: &["sda"],
    media_types: &["application/vnd.stardivision.draw"],
    internal_signatures: &[],
    related_formats: &[],
};
