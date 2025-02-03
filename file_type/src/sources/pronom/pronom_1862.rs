use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1862: FileFormat = FileFormat {
    id: 1_862,
    source_type: SourceType::Pronom,
    name: "SNAP Archive Data File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x53, 0x00, 0x75, 0x00, 0x72, 0x00, 0x76, 0x00, 0x65, 0x00, 0x79, 0x00,
                    ]),
                    Token::WildcardCountRange(0, 64),
                    Token::Literal(&[
                        0x00, 0x61, 0x00, 0x72, 0x00, 0x63, 0x00, 0x68, 0x00, 0x69, 0x00, 0x76,
                        0x00, 0x65, 0x00, 0x20, 0x00, 0x66, 0x00, 0x69, 0x00, 0x6C, 0x00, 0x65,
                        0x00,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
