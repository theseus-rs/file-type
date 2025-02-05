use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858769: FileFormat = FileFormat {
    id: 105_858_769,
    source_type: SourceType::Wikidata,
    name: "ADEX bitmap",
    extensions: &["img", "rle"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0x54, 0x00, 0x08])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0x54, 0x00, 0x08])],
                },
            }],
        },
    ],
    related_formats: &[],
};
