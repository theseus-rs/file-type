use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1344: FileFormat = FileFormat {
    id: 2_162,
    puid: "fmt/1344",
    name: "PTGui Project File",
    extensions: &["pts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6D, 0x2E, 0x70, 0x74, 0x67, 0x75, 0x69, 0x2E, 0x70, 0x72, 0x6F,
                    0x6A, 0x65, 0x63, 0x74, 0x5F, 0x70, 0x72, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
