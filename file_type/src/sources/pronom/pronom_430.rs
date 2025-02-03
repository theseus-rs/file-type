use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_430: FileFormat = FileFormat {
    id: 430,
    source_type: SourceType::Pronom,
    name: "Extensible Stylesheet Language",
    extensions: &["xsl"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
