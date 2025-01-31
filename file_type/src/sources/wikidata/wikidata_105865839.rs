use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865839: FileFormat = FileFormat {
    id: 105_865_839,
    puid: "wikidata/105865839",
    name: "Mobipocket - PRC Palm e-Book",
    extensions: &["mobi", "prc"],
    media_types: &[
        "application/x-mobipocket-ebook",
        "application/x-mobipocket-ebook",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
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
                        0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
