use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858639: FileFormat = FileFormat {
    id: 105_858_639,
    source_type: SourceType::Wikidata,
    name: "Award BIOS logo bitmap (v2)",
    extensions: &["bmp", "epa"],
    media_types: &["image/x-award-bioslogo2"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x42, 0x4D])],
                },
            }],
        },
        Signature {
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
