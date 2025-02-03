use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2160: FileFormat = FileFormat {
    id: 2_160,
    source_type: SourceType::Pronom,
    name: "BDOC",
    extensions: &["bdoc", "asice"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_389,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubtypeOf,
            id: 2_389,
        },
    ],
};
