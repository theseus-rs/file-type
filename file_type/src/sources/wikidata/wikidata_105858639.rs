use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858639: FileFormat = FileFormat {
    id: 105_858_639,
    puid: "wikidata/105858639",
    name: "Award BIOS logo bitmap (v2)",
    extensions: &["bmp", "epa"],
    media_types: &["image/x-award-bioslogo2", "image/x-award-bioslogo2"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x42, 0x4D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x42, 0x4D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
