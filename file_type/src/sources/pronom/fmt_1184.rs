use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1184: FileFormat = FileFormat {
    id: 1_994,
    puid: "fmt/1184",
    name: "InDesign Markup Language Package",
    extensions: &["idml"],
    media_types: &["application/vnd.adobe.indesign-idml-package"],
    internal_signatures: &[],
    related_formats: &[],
};
