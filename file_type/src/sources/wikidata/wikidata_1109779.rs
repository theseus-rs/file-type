use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1109779: FileFormat = FileFormat {
    id: 1_109_779,
    puid: "wikidata/1109779",
    name: "file shortcut",
    extensions: &["desktop", "desktop", "desktop", "lnk", "lnk", "lnk"],
    media_types: &[
        "text/plain",
        "text/plain",
        "text/plain",
        "text/plain",
        "text/plain",
        "text/plain",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00,
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
                        0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
