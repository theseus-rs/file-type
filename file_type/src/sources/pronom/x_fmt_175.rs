use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_175: FileFormat = FileFormat {
    id: 248,
    puid: "x-fmt/175",
    name: "MacPaint Graphics",
    extensions: &["pnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
