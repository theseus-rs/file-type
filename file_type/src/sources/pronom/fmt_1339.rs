use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1339: FileFormat = FileFormat {
    id: 2_157,
    puid: "fmt/1339",
    name: "PaperPort MAX",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x47, 0x46, 0x6B, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
