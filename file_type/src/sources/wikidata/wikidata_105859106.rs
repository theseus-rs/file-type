use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859106: FileFormat = FileFormat {
    id: 105_859_106,
    source_type: SourceType::Wikidata,
    name: "ImgStar bitmap",
    extensions: &["cpx", "flt"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4C, 0x69, 0x62])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4C, 0x69, 0x62])],
                },
            }],
        },
    ],
    related_formats: &[],
};
