use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_546: FileFormat = FileFormat {
    id: 1_334,
    puid: "fmt/546",
    name: "Macromedia FreeHand",
    extensions: &["fh9"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
