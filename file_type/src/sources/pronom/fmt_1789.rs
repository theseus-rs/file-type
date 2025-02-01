use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1789: FileFormat = FileFormat {
    id: 2_639,
    puid: "fmt/1789",
    name: "GX2 Graphics File",
    extensions: &["gx2", "ega"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
