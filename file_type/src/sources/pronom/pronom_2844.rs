use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2844: FileFormat = FileFormat {
    id: 2_844,
    source_type: SourceType::Pronom,
    name: "ICC Profile",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(8),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x05]),
                    Token::WildcardCount(27),
                    Token::Literal(&[0x61, 0x63, 0x73, 0x70]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_843,
    }],
};
