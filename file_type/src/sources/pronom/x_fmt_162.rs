use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_162: FileFormat = FileFormat {
    id: 229,
    puid: "x-fmt/162",
    name: "Adobe FrameMaker Interchange Format",
    extensions: &["mif"],
    media_types: &["application/vnd.mif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3C, 0x4D, 0x49, 0x46, 0x46, 0x69, 0x6C, 0x65, 0x20]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x3E, 0x20, 0x23]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
