use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1368: FileFormat = FileFormat {
    id: 2_186,
    puid: "fmt/1368",
    name: "Nero CoverDesigner File",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x56, 0x45, 0x52, 0x20, 0x45, 0x44, 0x49, 0x54, 0x4F, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
