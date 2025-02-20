use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2839: FileType = FileType {
    file_format: &FileFormat {
        id: 2_839,
        source_type: SourceType::Pronom,
        name: "Enigma Binary File (Finale)",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6E, 0x61, 0x6C, 0x65, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_145,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_837,
            },
        ],
    },
};
