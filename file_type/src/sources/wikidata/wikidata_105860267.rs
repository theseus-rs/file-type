use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860267: FileFormat = FileFormat {
    id: 105_860_267,
    puid: "wikidata/105860267",
    name: "ReDIF template",
    extensions: &["rdf", "redif"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x2D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x2D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
