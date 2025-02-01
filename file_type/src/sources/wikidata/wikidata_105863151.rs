use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863151: FileFormat = FileFormat {
    id: 105_863_151,
    puid: "wikidata/105863151",
    name: "Digitrax module",
    extensions: &["dtm", "mbm"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x42, 0x20, 0x4D, 0x4F, 0x44, 0x31, 0x40,
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
                        0x4D, 0x42, 0x20, 0x4D, 0x4F, 0x44, 0x31, 0x40,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
