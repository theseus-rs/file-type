use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852876: FileFormat = FileFormat {
    id: 105_852_876,
    puid: "wikidata/105852876",
    name: "Smart Software Time Manager Data",
    extensions: &["dtm", "stm"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x68, 0x75, 0x63, 0x6B, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x68, 0x75, 0x63, 0x6B, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
