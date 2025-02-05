use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2101: FileFormat = FileFormat {
    id: 2_101,
    source_type: SourceType::Pronom,
    name: "PFS:First Choice Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x4F, 0x43, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_100,
    }],
};
