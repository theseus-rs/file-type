use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1994: FileFormat = FileFormat {
    id: 1_994,
    source_type: SourceType::Pronom,
    name: "InDesign Markup Language Package",
    extensions: &["idml"],
    media_types: &["application/vnd.adobe.indesign-idml-package"],
    signatures: &[],
    related_formats: &[],
};
