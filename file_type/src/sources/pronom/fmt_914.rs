use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_914: FileFormat = FileFormat {
    id: 1_719,
    puid: "fmt/914",
    name: "Caligari trueSpace File Format",
    extensions: &["cob", "scn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x6C, 0x69, 0x67, 0x61, 0x72, 0x69, 0x20, 0x56, 0x30, 0x30, 0x2E,
                    0x30, 0x31, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
