use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1471: FileFormat = FileFormat {
    id: 2_294,
    puid: "fmt/1471",
    name: "Multi Palette Picture File",
    extensions: &["mpp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
