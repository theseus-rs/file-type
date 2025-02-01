use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_717: FileFormat = FileFormat {
    id: 1_516,
    puid: "fmt/717",
    name: "Scream Tracker Module",
    extensions: &["stm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(20),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x53, 0x63, 0x72, 0x65, 0x61, 0x6D, 0x21, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
