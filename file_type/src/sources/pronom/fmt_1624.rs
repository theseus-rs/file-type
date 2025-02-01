use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1624: FileFormat = FileFormat {
    id: 2_451,
    puid: "fmt/1624",
    name: "Art Of Noise",
    extensions: &["aon"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
