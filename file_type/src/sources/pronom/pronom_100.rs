use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_100: FileFormat = FileFormat {
    id: 100,
    source_type: SourceType::Pronom,
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
