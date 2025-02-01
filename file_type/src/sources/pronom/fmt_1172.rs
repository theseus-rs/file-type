use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1172: FileFormat = FileFormat {
    id: 1_982,
    puid: "fmt/1172",
    name: "Web Open Font Format",
    extensions: &["woff2"],
    media_types: &["font/woff2"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x4F, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
