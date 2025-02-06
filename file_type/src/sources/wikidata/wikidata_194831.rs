use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_194831: FileFormat = FileFormat {
    id: 194_831,
    source_type: SourceType::Wikidata,
    name: "Cabinet",
    extensions: &["cab"],
    media_types: &["application/vnd.ms-cab-compressed"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x43, 0x46])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
