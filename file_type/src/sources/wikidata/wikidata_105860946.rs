use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860946: FileFormat = FileFormat {
    id: 105_860_946,
    puid: "wikidata/105860946",
    name: "CODESYS Library",
    extensions: &["lb6", "lbx"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x44, 0x65, 0x53, 0x79, 0x73, 0x2B, 0x0F, 0x17, 0x8C,
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
                        0x43, 0x6F, 0x44, 0x65, 0x53, 0x79, 0x73, 0x2B, 0x0F, 0x17, 0x8C,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
