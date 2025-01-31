use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_162839: FileFormat = FileFormat {
    id: 162_839,
    puid: "wikidata/162839",
    name: "xz",
    extensions: &["xz", "xz"],
    media_types: &["application/x-xz", "application/x-xz"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x5A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
