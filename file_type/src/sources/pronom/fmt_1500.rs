use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1500: FileFormat = FileFormat {
    id: 2_323,
    puid: "fmt/1500",
    name: "Adobe Acrobat Forms Data Format",
    extensions: &["fdf"],
    media_types: &["application/vnd.fdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x46, 0x44, 0x46, 0x2D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
