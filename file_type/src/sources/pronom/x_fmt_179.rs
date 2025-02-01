use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_179: FileFormat = FileFormat {
    id: 252,
    puid: "x-fmt/179",
    name: "Microsoft Visual Modeller Petal file (ASCII)",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
