use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_508: FileFormat = FileFormat {
    id: 508,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database",
    extensions: &["bdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
