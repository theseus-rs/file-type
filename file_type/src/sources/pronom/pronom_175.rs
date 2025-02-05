use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_175: FileFormat = FileFormat {
    id: 175,
    source_type: SourceType::Pronom,
    name: "Microsoft Works for Windows",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
