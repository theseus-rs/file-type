use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_94: FileFormat = FileFormat {
    id: 141,
    puid: "x-fmt/94",
    name: "Pocket Word Document",
    extensions: &["psw", "pwd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x5C, 0x70, 0x77, 0x64, 0x32, 0x5C, 0x61, 0x6E, 0x73, 0x69,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
