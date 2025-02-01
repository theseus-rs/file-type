use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_123: FileFormat = FileFormat {
    id: 178,
    puid: "x-fmt/123",
    name: "Microsoft Excel Macro",
    extensions: &["xla", "xlm"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x04, 0x06, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x40, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
