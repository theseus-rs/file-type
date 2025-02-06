use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205733: FileFormat = FileFormat {
    id: 28_205_733,
    source_type: SourceType::Wikidata,
    name: "Award BIOS logo, version 1",
    extensions: &["bmp", "epa"],
    media_types: &["image/x-award-bioslogo"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x09])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x09])],
                },
            }],
        },
    ],
    related_formats: &[],
};
