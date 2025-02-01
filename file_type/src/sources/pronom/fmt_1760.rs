use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1760: FileFormat = FileFormat {
    id: 2_610,
    puid: "fmt/1760",
    name: "CloneCD Control File",
    extensions: &["ccd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x6C, 0x6F, 0x6E, 0x65, 0x43, 0x44, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
