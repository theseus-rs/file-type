use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2076: FileFormat = FileFormat {
    id: 2_076,
    source_type: SourceType::Pronom,
    name: "Microsoft Access Workgroup Information File",
    extensions: &["mdw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x65, 0x74, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20, 0x44, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
