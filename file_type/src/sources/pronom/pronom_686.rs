use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_686: FileFormat = FileFormat {
    id: 686,
    source_type: SourceType::Pronom,
    name: "JP2 (JPEG 2000 part 1)",
    extensions: &["jp2"],
    media_types: &["image/jp2"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x6A, 0x70, 0x32]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
