use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_263: FileFormat = FileFormat {
    id: 263,
    source_type: SourceType::Pronom,
    name: "RealMedia",
    extensions: &["rm", "rmvb"],
    media_types: &["application/vnd.rn-realmedia"],
    signatures: &[Signature {
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
