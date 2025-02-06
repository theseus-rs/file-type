use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856307: FileFormat = FileFormat {
    id: 105_856_307,
    source_type: SourceType::Wikidata,
    name: "Micrografx Designer Drawing (v3.x)",
    extensions: &["drw", "dsf"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xFF, 0x02, 0x04, 0x03])],
                },
            }],
        },
        Signature {
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
