use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_100: FileFormat = FileFormat {
    id: 100,
    source_type: SourceType::Pronom,
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
