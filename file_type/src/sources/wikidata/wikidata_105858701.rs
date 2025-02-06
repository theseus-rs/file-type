use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858701: FileFormat = FileFormat {
    id: 105_858_701,
    source_type: SourceType::Wikidata,
    name: "Paintpro bitmap (v5.0)",
    extensions: &["ppp", "tb1"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x4F, 0x56, 0x35, 0x2E, 0x30,
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
                        0x50, 0x41, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x4F, 0x56, 0x35, 0x2E, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
