use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1729: FileFormat = FileFormat {
    id: 1_729,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Drawing",
    extensions: &["vsdx"],
    media_types: &["application/vnd.ms-visio.drawing.main+xml"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_230,
    }],
};
