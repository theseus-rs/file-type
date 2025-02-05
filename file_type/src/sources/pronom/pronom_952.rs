use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_952: FileFormat = FileFormat {
    id: 952,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
