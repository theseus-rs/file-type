use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1679: FileFormat = FileFormat {
    id: 1_679,
    source_type: SourceType::Pronom,
    name: "RDF/XML",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    signatures: &[],
    related_formats: &[],
};
