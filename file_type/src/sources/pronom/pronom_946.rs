use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_946: FileFormat = FileFormat {
    id: 946,
    source_type: SourceType::Pronom,
    name: "Microsoft FrontPage",
    extensions: &["lck"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
