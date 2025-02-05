use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_250: FileFormat = FileFormat {
    id: 250,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Graphics File",
    extensions: &["ppi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
