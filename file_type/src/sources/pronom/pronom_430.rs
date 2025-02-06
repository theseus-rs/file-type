use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_430: FileFormat = FileFormat {
    id: 430,
    source_type: SourceType::Pronom,
    name: "Extensible Stylesheet Language",
    extensions: &["xsl"],
    media_types: &["application/xml"],
    signatures: &[],
    related_formats: &[],
};
