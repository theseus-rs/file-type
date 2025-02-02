use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1068: FileFormat = FileFormat {
    id: 1_068,
    source_type: SourceType::Pronom,
    name: "Extended Module Audio File",
    extensions: &["xm"],
    media_types: &["audio/xm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x45, 0x78, 0x74, 0x65, 0x6E, 0x64, 0x65, 0x64, 0x20, 0x4D, 0x6F, 0x64,
                        0x75, 0x6C, 0x65, 0x3A, 0x20,
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x1A]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
