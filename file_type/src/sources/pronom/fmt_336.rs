use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_336: FileFormat = FileFormat {
    id: 1_081,
    puid: "fmt/336",
    name: "Graphic Workshop for Windows Thumbnail File",
    extensions: &["thn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x4E, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
