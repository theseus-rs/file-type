use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_59: FileFormat = FileFormat {
    id: 100,
    puid: "x-fmt/59",
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
