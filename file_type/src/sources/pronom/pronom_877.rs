use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_877: FileFormat = FileFormat {
    id: 877,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel for Macintosh",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
