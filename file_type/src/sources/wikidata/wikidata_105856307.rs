use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856307: FileFormat = FileFormat {
    id: 105_856_307,
    puid: "wikidata/105856307",
    name: "Micrografx Designer Drawing (v3.x)",
    extensions: &["drw", "dsf"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03])],
                },
            }],
        },
    ],
    related_formats: &[],
};
