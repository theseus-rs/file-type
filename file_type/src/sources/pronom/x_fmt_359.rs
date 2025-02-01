use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_359: FileFormat = FileFormat {
    id: 525,
    puid: "x-fmt/359",
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
