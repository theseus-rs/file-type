use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1447: FileFormat = FileFormat {
    id: 2_265,
    puid: "fmt/1447",
    name: "XLD4 (Bitmap Image)",
    extensions: &["q4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x4A, 0x59, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
