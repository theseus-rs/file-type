use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1580: FileFormat = FileFormat {
    id: 2_405,
    puid: "fmt/1580",
    name: "Envision Publisher File",
    extensions: &["evp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6E, 0x56, 0x69, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x50, 0x75, 0x62, 0x6C,
                    0x69, 0x73, 0x68, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
