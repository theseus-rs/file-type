use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1674: FileFormat = FileFormat {
    id: 1_674,
    source_type: SourceType::Pronom,
    name: "Perl Script",
    extensions: &["pl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
