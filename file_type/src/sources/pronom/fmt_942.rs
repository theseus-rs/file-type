use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_942: FileFormat = FileFormat {
    id: 1_747,
    puid: "fmt/942",
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
