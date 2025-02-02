use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_240: FileFormat = FileFormat {
    id: 240,
    source_type: SourceType::Pronom,
    name: "Broderbund Print Shop Deluxe",
    extensions: &["pcc", "pdb", "pdc", "pda", "pdl", "pds", "pdg"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x69, 0x6E, 0x74, 0x53, 0x68, 0x6F, 0x70, 0x44, 0x65, 0x6C,
                        0x75, 0x78, 0x65,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_117,
    }],
};
