use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_264627: FileFormat = FileFormat {
    id: 264_627,
    puid: "wikidata/264627",
    name: "Glulx",
    extensions: &["blb", "blorb", "gblorb", "glb", "ulx"],
    media_types: &[
        "application/x-glulx",
        "application/x-glulx",
        "application/x-glulx",
        "application/x-glulx",
        "application/x-glulx",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x6C, 0x75, 0x6C])],
                },
            }],
        },
    ],
    related_formats: &[],
};
