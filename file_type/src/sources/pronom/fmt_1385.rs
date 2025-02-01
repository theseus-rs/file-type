use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1385: FileFormat = FileFormat {
    id: 2_203,
    puid: "fmt/1385",
    name: "Bruker PDZ",
    extensions: &["pdz", "xpdz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x01, 0x17, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
