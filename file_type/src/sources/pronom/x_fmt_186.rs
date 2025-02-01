use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_186: FileFormat = FileFormat {
    id: 259,
    puid: "x-fmt/186",
    name: "Silicon Graphics RGB File",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
