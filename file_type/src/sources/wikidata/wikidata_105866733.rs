use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866733: FileFormat = FileFormat {
    id: 105_866_733,
    puid: "wikidata/105866733",
    name: "First Choice document",
    extensions: &["doc", "pfs"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x4F, 0x43,
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
                        0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x4F, 0x43,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
