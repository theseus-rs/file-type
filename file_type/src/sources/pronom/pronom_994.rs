use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_994: FileFormat = FileFormat {
    id: 994,
    source_type: SourceType::Pronom,
    name: "Microsoft Works Database for Windows",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
