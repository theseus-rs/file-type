use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_611: FileFormat = FileFormat {
    id: 1_407,
    puid: "fmt/611",
    name: "LDAP Data Interchange Format",
    extensions: &["ldif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x6E, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
