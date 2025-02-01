use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864206: FileFormat = FileFormat {
    id: 105_864_206,
    puid: "wikidata/105864206",
    name: "PGP message",
    extensions: &["asc", "pgp", "txt"],
    media_types: &["text/plain", "text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50,
                        0x47, 0x50, 0x20, 0x4D, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x2D, 0x2D,
                        0x2D, 0x2D, 0x2D,
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
                        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50,
                        0x47, 0x50, 0x20, 0x4D, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x2D, 0x2D,
                        0x2D, 0x2D, 0x2D,
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
                        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50,
                        0x47, 0x50, 0x20, 0x4D, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x2D, 0x2D,
                        0x2D, 0x2D, 0x2D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
