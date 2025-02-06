use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2121: FileFormat = FileFormat {
    id: 2_121,
    source_type: SourceType::Pronom,
    name: "Microsoft Shell Scrap Object File",
    extensions: &["shs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
