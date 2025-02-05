use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_178: FileFormat = FileFormat {
    id: 178,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Macro",
    extensions: &["xla", "xlm"],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[Signature {
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
