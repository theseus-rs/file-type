use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858396: FileFormat = FileFormat {
    id: 105_858_396,
    source_type: SourceType::Wikidata,
    name: "Extron IP Link driver",
    extensions: &["pke", "pkn"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x54, 0x58, 0x45, 0x47, 0x4B, 0x50, 0x45,
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
                        0x4E, 0x54, 0x58, 0x45, 0x47, 0x4B, 0x50, 0x45,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
