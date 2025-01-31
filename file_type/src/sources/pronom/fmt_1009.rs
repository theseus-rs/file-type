use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1009: FileFormat = FileFormat {
    id: 1_814,
    puid: "fmt/1009",
    name: "FBX (Filmbox) Binary",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x61, 0x79, 0x64, 0x61, 0x72, 0x61, 0x20, 0x46, 0x42, 0x58, 0x20, 0x42,
                    0x69, 0x6E, 0x61, 0x72, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
