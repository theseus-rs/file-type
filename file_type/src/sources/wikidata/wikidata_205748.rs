use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_205748: FileFormat = FileFormat {
    id: 205_748,
    source_type: SourceType::Wikidata,
    name: "Portable Game Notation",
    extensions: &["pgn"],
    media_types: &["application/vnd.chess-pgn", "application/x-chess-pgn"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x20, 0x22,
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
                        0x5B, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x20, 0x22,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
