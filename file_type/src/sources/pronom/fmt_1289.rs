use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1289: FileFormat = FileFormat {
    id: 2_107,
    puid: "fmt/1289",
    name: "RFFlow Chart",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x46, 0x4C, 0x4F, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
