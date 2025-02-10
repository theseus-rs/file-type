use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1066897: FileType = FileType {
    file_format: &FileFormat {
        id: 1_066_897,
        source_type: SourceType::Wikidata,
        name: "LDAP Data Interchange Format",
        extensions: &["ldif"],
        media_types: &["text/plain", "text/x-ldif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x6E, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
