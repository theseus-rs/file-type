use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1225: FileFormat = FileFormat {
    id: 2_035,
    puid: "fmt/1225",
    name: "PaperPort MAX",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x45, 0x6D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
