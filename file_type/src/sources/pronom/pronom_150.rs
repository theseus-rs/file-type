use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_150: FileFormat = FileFormat {
    id: 150,
    source_type: SourceType::Pronom,
    name: "3D Studio (DOS) 2D Shape File",
    extensions: &["shp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x2D, 0x2D]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
