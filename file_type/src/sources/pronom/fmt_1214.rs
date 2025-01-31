use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1214: FileFormat = FileFormat {
    id: 2_024,
    puid: "fmt/1214",
    name: "Cakewalk WRK Project",
    extensions: &["wrk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4B, 0x45, 0x57, 0x41, 0x4C, 0x4B, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
