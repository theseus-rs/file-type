use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1066897: FileFormat = FileFormat {
    id: 1_066_897,
    puid: "wikidata/1066897",
    name: "LDAP Data Interchange Format",
    extensions: &["ldif", "ldif"],
    media_types: &["text/plain", "text/x-ldif"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x6E, 0x3A, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x6E, 0x3A, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
