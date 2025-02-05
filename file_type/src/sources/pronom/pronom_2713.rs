use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2713: FileFormat = FileFormat {
    id: 2_713,
    source_type: SourceType::Pronom,
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    signatures: &[],
    related_formats: &[],
};
