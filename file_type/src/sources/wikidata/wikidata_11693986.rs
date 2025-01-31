use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11693986: FileFormat = FileFormat {
    id: 11_693_986,
    puid: "wikidata/11693986",
    name: "Desktop Entry",
    extensions: &["desktop", "desktop"],
    media_types: &["application/x-desktop", "text/ini"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x44, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x20, 0x45, 0x6E, 0x74,
                        0x72, 0x79, 0x5D,
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
                        0x5B, 0x44, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x20, 0x45, 0x6E, 0x74,
                        0x72, 0x79, 0x5D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
