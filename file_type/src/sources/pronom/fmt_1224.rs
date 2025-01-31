use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1224: FileFormat = FileFormat {
    id: 2_034,
    puid: "fmt/1224",
    name: "PaperPort MAX",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x43, 0x6A, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
