use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1881: FileFormat = FileFormat {
    id: 1_881,
    source_type: SourceType::Pronom,
    name: "AVCHD Playlist File",
    extensions: &["mpl", "mpls"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x50, 0x4C, 0x53, 0x30, 0x31, 0x30, 0x30,
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
                        0x4D, 0x50, 0x4C, 0x53, 0x30, 0x32, 0x30, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
