use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861099: FileFormat = FileFormat {
    id: 105_861_099,
    source_type: SourceType::Wikidata,
    name: "LabVIEW binary Datalog",
    extensions: &["dat", "log"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x4C, 0x47])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x4C, 0x47])],
                },
            }],
        },
    ],
    related_formats: &[],
};
