use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852340: FileFormat = FileFormat {
    id: 105_852_340,
    puid: "wikidata/105852340",
    name: "XRoar Snapshot",
    extensions: &["snap", "snp"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x52, 0x6F, 0x61, 0x72, 0x20, 0x73, 0x6E, 0x61, 0x70, 0x73, 0x68,
                        0x6F, 0x74, 0x2E, 0x0A,
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
                        0x58, 0x52, 0x6F, 0x61, 0x72, 0x20, 0x73, 0x6E, 0x61, 0x70, 0x73, 0x68,
                        0x6F, 0x74, 0x2E, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
