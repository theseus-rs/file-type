use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_427: FileFormat = FileFormat {
    id: 814,
    puid: "x-fmt/427",
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
