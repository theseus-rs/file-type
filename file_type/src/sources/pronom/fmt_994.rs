use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_994: FileFormat = FileFormat {
    id: 1_799,
    puid: "fmt/994",
    name: "Jeffs Image Format",
    extensions: &["jif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x49, 0x46, 0x39, 0x39, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
