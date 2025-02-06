use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967218: FileFormat = FileFormat {
    id: 27_967_218,
    source_type: SourceType::Wikidata,
    name: "Scream Tracker 1 & 2 module",
    extensions: &["stm", "stx"],
    media_types: &["audio/x-mod"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x53, 0x63, 0x72, 0x65, 0x61, 0x6D, 0x21,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x53, 0x63, 0x72, 0x65, 0x61, 0x6D, 0x21,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
