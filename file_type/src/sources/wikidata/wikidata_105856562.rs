use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856562: FileFormat = FileFormat {
    id: 105_856_562,
    puid: "wikidata/105856562",
    name: "WordStar 5.5 document",
    extensions: &["doc", "ws5"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x55])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x55])],
                },
            }],
        },
    ],
    related_formats: &[],
};
