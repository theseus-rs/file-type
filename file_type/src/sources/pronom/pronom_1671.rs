use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1671: FileFormat = FileFormat {
    id: 1_671,
    source_type: SourceType::Pronom,
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
