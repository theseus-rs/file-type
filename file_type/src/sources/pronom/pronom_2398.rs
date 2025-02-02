use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2398: FileFormat = FileFormat {
    id: 2_398,
    source_type: SourceType::Pronom,
    name: "Visual Basic Project File",
    extensions: &["vbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x79, 0x70, 0x65, 0x3D, 0x45, 0x78, 0x65, 0x0D, 0x0A,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x4D, 0x61, 0x78, 0x4E, 0x75, 0x6D, 0x62, 0x65, 0x72, 0x4F, 0x66, 0x54,
                            0x68, 0x72, 0x65, 0x61, 0x64, 0x73, 0x3D,
                        ]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x0D, 0x0A]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
