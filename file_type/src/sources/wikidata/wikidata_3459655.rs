use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3459655: FileFormat = FileFormat {
    id: 3_459_655,
    puid: "wikidata/3459655",
    name: "StuffIt X archive",
    extensions: &["sitx", "sitx"],
    media_types: &["application/x-sitx", "application/x-stuffitx"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x21,
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
                        0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x21,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
