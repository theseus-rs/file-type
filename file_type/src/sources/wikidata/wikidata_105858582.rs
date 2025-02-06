use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858582: FileFormat = FileFormat {
    id: 105_858_582,
    source_type: SourceType::Wikidata,
    name: "Cloe picture bitmap (little endian)",
    extensions: &["clo", "cloe"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x4D, 0x4D])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x4D, 0x4D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
