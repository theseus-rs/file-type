use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50562863: FileFormat = FileFormat {
    id: 50_562_863,
    source_type: SourceType::Wikidata,
    name: "Windows Journal Format",
    extensions: &["jnt", "jtp"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x42, 0x2A, 0x00])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x42, 0x2A, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
