use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853361: FileFormat = FileFormat {
    id: 105_853_361,
    source_type: SourceType::Wikidata,
    name: "SquashSF image file (big endian)",
    extensions: &["sfs", "squashfs"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x71, 0x73, 0x68])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x71, 0x73, 0x68])],
                },
            }],
        },
    ],
    related_formats: &[],
};
