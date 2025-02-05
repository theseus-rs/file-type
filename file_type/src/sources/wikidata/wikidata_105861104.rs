use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861104: FileFormat = FileFormat {
    id: 105_861_104,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts interactive sequence",
    extensions: &["lin", "map"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x44, 0x78, 0x00])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x44, 0x78, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
