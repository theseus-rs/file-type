use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_954: FileFormat = FileFormat {
    id: 954,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
