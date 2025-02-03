use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2362: FileFormat = FileFormat {
    id: 2_362,
    source_type: SourceType::Pronom,
    name: "CompuServe RLE",
    extensions: &["rle"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1B, 0x47]),
                    Token::Any(&[&[Token::Literal(&[0x48])], &[Token::Literal(&[0x4D])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
