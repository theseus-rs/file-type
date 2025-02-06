use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862800: FileFormat = FileFormat {
    id: 105_862_800,
    source_type: SourceType::Wikidata,
    name: "Rdos Raw OPL Capture music",
    extensions: &["rac", "raw"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x41, 0x57, 0x41, 0x44, 0x41, 0x54, 0x41,
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
                        0x52, 0x41, 0x57, 0x41, 0x44, 0x41, 0x54, 0x41,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
