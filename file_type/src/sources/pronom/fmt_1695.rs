use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1695: FileFormat = FileFormat {
    id: 2_531,
    puid: "fmt/1695",
    name: "602 Text file",
    extensions: &["602"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x43, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
