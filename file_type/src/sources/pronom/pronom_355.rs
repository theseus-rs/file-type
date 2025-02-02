use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_355: FileFormat = FileFormat {
    id: 355,
    source_type: SourceType::Pronom,
    name: "Microsoft FoxPro Database",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
