use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_545: FileFormat = FileFormat {
    id: 1_333,
    puid: "fmt/545",
    name: "Macromedia FreeHand",
    extensions: &["fh8"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
