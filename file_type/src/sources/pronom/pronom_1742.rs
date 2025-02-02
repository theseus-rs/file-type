use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1742: FileFormat = FileFormat {
    id: 1_742,
    source_type: SourceType::Pronom,
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
