use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856517: FileFormat = FileFormat {
    id: 105_856_517,
    puid: "wikidata/105856517",
    name: "WordStar 5 document",
    extensions: &["doc", "ws5"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x50])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x50])],
                },
            }],
        },
    ],
    related_formats: &[],
};
