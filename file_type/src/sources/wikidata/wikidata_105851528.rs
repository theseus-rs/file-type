use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851528: FileFormat = FileFormat {
    id: 105_851_528,
    puid: "wikidata/105851528",
    name: "WinHex data structure template",
    extensions: &["tpl", "txt"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x74, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x20, 0x22,
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
                        0x74, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x20, 0x22,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
