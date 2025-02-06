use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854338: FileFormat = FileFormat {
    id: 105_854_338,
    source_type: SourceType::Wikidata,
    name: "Gold Disk Office Calc/Graph spreadsheet",
    extensions: &["adv", "pcf"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x04, 0x28, 0x9B, 0x86, 0xF4, 0x02, 0x00, 0x08,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x04, 0x28, 0x9B, 0x86, 0xF4, 0x02, 0x00, 0x08,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
