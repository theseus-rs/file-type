use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_867: FileFormat = FileFormat {
    id: 1_671,
    puid: "fmt/867",
    name: "Microsoft Reader eBook",
    extensions: &["lit"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x54, 0x4F, 0x4C, 0x49, 0x54, 0x4C, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
