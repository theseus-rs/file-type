use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_889: FileFormat = FileFormat {
    id: 889,
    source_type: SourceType::Pronom,
    name: "Microsoft Multiplan",
    extensions: &["mod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
