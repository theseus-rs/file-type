use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1712: FileFormat = FileFormat {
    id: 2_548,
    puid: "fmt/1712",
    name: "Calc602 Macro File",
    extensions: &["mc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
