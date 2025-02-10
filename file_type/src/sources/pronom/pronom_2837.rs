use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2837: FileType = FileType {
    file_format: &FileFormat {
        id: 2_837,
        source_type: SourceType::Pronom,
        name: "Enigma Binary File (Finale)",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x69, 0x6E, 0x61, 0x6C, 0x65]),
                        Token::Any(&[
                            &[Token::Literal(&[0xAA])],
                            &[Token::Literal(&[0x28, 0x54, 0x4D, 0x29])],
                        ]),
                        Token::Literal(&[0x20, 0x31]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_839,
        }],
    },
};
