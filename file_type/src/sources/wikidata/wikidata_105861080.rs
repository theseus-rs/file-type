use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861080: FileFormat = FileFormat {
    id: 105_861_080,
    puid: "wikidata/105861080",
    name: "Windows application log",
    extensions: &["lgc", "lgd"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0D, 0x0A, 0x6F, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0D, 0x0A, 0x6F, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
