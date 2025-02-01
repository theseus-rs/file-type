use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1579: FileFormat = FileFormat {
    id: 2_404,
    puid: "fmt/1579",
    name: "SPYne Containers",
    extensions: &["spy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xA7, 0x02, 0x20, 0xA0, 0xE5, 0xA9, 0x0A, 0x8D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
