use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1787: FileFormat = FileFormat {
    id: 2_637,
    puid: "fmt/1787",
    name: "G9B Graphics Format Bitmap",
    extensions: &["g9b"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x39, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
