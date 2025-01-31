use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1617: FileFormat = FileFormat {
    id: 2_444,
    puid: "fmt/1617",
    name: "Devicetree Blob (DTB)",
    extensions: &["dtb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD0, 0x0D, 0xFE, 0xED])],
            },
        }],
    }],
    related_formats: &[],
};
