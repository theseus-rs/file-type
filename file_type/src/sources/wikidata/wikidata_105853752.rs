use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853752: FileFormat = FileFormat {
    id: 105_853_752,
    puid: "wikidata/105853752",
    name: "WRAptor compressed",
    extensions: &["wr3", "wra"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x42, 0x4C, 0xFF])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x42, 0x4C, 0xFF])],
                },
            }],
        },
    ],
    related_formats: &[],
};
