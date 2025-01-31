use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1674: FileFormat = FileFormat {
    id: 2_510,
    puid: "fmt/1674",
    name: "ZyXEL Voice Format Audio",
    extensions: &["zvd", "zyx", "ad2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
