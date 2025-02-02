use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_250: FileFormat = FileFormat {
    id: 250,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Graphics File",
    extensions: &["ppi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
