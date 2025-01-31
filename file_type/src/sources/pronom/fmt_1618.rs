use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1618: FileFormat = FileFormat {
    id: 2_445,
    puid: "fmt/1618",
    name: "SGML/XML Entity File",
    extensions: &["ent"],
    media_types: &["application/xml-external-parsed-entity"],
    internal_signatures: &[],
    related_formats: &[],
};
