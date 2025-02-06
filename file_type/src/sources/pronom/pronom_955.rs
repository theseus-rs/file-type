use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_955: FileFormat = FileFormat {
    id: 955,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
