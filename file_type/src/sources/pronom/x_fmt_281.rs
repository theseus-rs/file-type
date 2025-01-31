use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_281: FileFormat = FileFormat {
    id: 430,
    puid: "x-fmt/281",
    name: "Extensible Stylesheet Language",
    extensions: &["xsl"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
