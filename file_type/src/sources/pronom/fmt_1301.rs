use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1301: FileFormat = FileFormat {
    id: 2_119,
    puid: "fmt/1301",
    name: "The Print Shop Project",
    extensions: &["psproj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(12),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x35, 0x52, 0x44, 0x4F, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
