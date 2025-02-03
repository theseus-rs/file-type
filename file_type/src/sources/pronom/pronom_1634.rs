use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1634: FileFormat = FileFormat {
    id: 1_634,
    source_type: SourceType::Pronom,
    name: "Open Inventor File Format",
    extensions: &["iv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x23, 0x49, 0x6E, 0x76, 0x65, 0x6E, 0x74, 0x6F, 0x72, 0x20, 0x56, 0x32,
                        0x2E,
                    ]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61, 0x73, 0x63, 0x69, 0x69])],
                        &[Token::Literal(&[0x62, 0x69, 0x6E, 0x61, 0x72, 0x79])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_633,
    }],
};
