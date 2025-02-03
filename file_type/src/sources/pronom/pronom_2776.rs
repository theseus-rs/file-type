use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2776: FileFormat = FileFormat {
    id: 2_776,
    source_type: SourceType::Pronom,
    name: "MetaCard Stack",
    extensions: &["mc", "rev"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x21, 0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x73, 0x68, 0x0A, 0x23, 0x20, 0x4D,
                    0x65, 0x74, 0x61, 0x43, 0x61, 0x72, 0x64, 0x20, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
