use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_190: FileFormat = FileFormat {
    id: 190,
    source_type: SourceType::Pronom,
    name: "Stationery for Mac OS X",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
