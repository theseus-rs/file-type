use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855429: FileFormat = FileFormat {
    id: 105_855_429,
    puid: "wikidata/105855429",
    name: "Gherkin Feature",
    extensions: &["feature", "feature"],
    media_types: &["text/plain", "text/x-gherkin"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x3A,
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
                        0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x3A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
