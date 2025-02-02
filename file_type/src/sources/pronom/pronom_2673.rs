use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2673: FileFormat = FileFormat {
    id: 2_673,
    source_type: SourceType::Pronom,
    name: "Audacity Audio Block File",
    extensions: &["au"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Any(&[
                        &[Token::Literal(&[0x2E, 0x73, 0x6E, 0x64])],
                        &[Token::Literal(&[0x64, 0x6E, 0x73, 0x2E])],
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[
                        0x41, 0x75, 0x64, 0x61, 0x63, 0x69, 0x74, 0x79, 0x42, 0x6C, 0x6F, 0x63,
                        0x6B, 0x46, 0x69, 0x6C, 0x65,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
