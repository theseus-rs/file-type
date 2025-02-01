use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_75: FileFormat = FileFormat {
    id: 117,
    puid: "x-fmt/75",
    name: "Microsoft Outlook Personal Address Book",
    extensions: &["pab"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x42, 0x44, 0x4E]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x41, 0x42]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
