use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_190: FileFormat = FileFormat {
    id: 263,
    puid: "x-fmt/190",
    name: "RealMedia",
    extensions: &["rm", "rmvb"],
    media_types: &["application/vnd.rn-realmedia"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x2E, 0x52, 0x4D, 0x46, 0x00, 0x00, 0x00]),
                    Token::Any(&[&[Token::Literal(&[0x10])], &[Token::Literal(&[0x12])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
