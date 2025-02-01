use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856750: FileFormat = FileFormat {
    id: 105_856_750,
    puid: "wikidata/105856750",
    name: "Universal Go Format",
    extensions: &["ugf", "ugi"],
    media_types: &["text/ini", "text/ini"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5D,
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
                        0x5B, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
