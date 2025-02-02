use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1691: FileFormat = FileFormat {
    id: 1_691,
    source_type: SourceType::Pronom,
    name: "SafeGuard Encrypted Virtual Disk",
    extensions: &["vol", "hdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x44, 0x69, 0x73, 0x6B, 0x20, 0x56,
                    0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x44, 0x69, 0x73, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
