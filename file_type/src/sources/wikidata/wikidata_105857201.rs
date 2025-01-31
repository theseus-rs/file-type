use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857201: FileFormat = FileFormat {
    id: 105_857_201,
    puid: "wikidata/105857201",
    name: "HP Printer Command Language (ESC+E)",
    extensions: &["pcl", "prn"],
    media_types: &["application/vnd.hp-PCL", "application/vnd.hp-PCL"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x45])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x45])],
                },
            }],
        },
    ],
    related_formats: &[],
};
