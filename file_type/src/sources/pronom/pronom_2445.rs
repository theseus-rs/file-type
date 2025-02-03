use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2445: FileFormat = FileFormat {
    id: 2_445,
    source_type: SourceType::Pronom,
    name: "SGML/XML Entity File",
    extensions: &["ent"],
    media_types: &["application/xml-external-parsed-entity"],
    internal_signatures: &[],
    related_formats: &[],
};
