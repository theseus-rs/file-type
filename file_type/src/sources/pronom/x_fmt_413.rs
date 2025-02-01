use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_413: FileFormat = FileFormat {
    id: 800,
    puid: "x-fmt/413",
    name: "Batch file (executable)",
    extensions: &["bat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
