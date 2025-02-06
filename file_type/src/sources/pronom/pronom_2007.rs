use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2007: FileFormat = FileFormat {
    id: 2_007,
    source_type: SourceType::Pronom,
    name: "MyISAM Indexes File",
    extensions: &["myi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFE, 0xFE, 0x07, 0x01]),
                    Token::WildcardCount(19),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
