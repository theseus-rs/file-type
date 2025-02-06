use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_355: FileFormat = FileFormat {
    id: 355,
    source_type: SourceType::Pronom,
    name: "Microsoft FoxPro Database",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
