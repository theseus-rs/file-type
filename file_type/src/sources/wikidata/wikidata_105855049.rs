use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855049: FileFormat = FileFormat {
    id: 105_855_049,
    source_type: SourceType::Wikidata,
    name: "AY chiptune",
    extensions: &["ay", "emul"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x41, 0x59, 0x45, 0x4D, 0x55, 0x4C,
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
                        0x5A, 0x58, 0x41, 0x59, 0x45, 0x4D, 0x55, 0x4C,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
