use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853767: FileFormat = FileFormat {
    id: 105_853_767,
    source_type: SourceType::Wikidata,
    name: "Sudden Strike 2 game data archive",
    extensions: &["aps", "sue"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x5A, 0x46, 0x46])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x5A, 0x46, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
