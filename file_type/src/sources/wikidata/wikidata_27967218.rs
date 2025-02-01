use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967218: FileFormat = FileFormat {
    id: 27_967_218,
    puid: "wikidata/27967218",
    name: "Scream Tracker 1 & 2 module",
    extensions: &["stm", "stx"],
    media_types: &["audio/x-mod", "audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
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
        InternalSignature {
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
