use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858947: FileFormat = FileFormat {
    id: 105_858_947,
    source_type: SourceType::Wikidata,
    name: "Cartesian Perceptual Compression Image bitmap",
    extensions: &["cpc", "cpi"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x43, 0xB2])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x43, 0xB2])],
                },
            }],
        },
    ],
    related_formats: &[],
};
