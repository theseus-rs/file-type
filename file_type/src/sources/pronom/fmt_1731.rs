use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1731: FileFormat = FileFormat {
    id: 2_576,
    puid: "fmt/1731",
    name: "PowerGraphics Image File",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(8),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x77, 0x65, 0x72, 0x47, 0x46, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
