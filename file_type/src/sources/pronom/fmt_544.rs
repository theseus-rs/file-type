use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_544: FileFormat = FileFormat {
    id: 1_331,
    puid: "fmt/544",
    name: "Macromedia FreeHand",
    extensions: &["fh7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x47, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
