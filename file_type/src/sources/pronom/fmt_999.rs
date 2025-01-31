use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_999: FileFormat = FileFormat {
    id: 1_804,
    puid: "fmt/999",
    name: "Krita Document Format",
    extensions: &["kra"],
    media_types: &["application/x-krita"],
    internal_signatures: &[],
    related_formats: &[],
};
