use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_477: FileFormat = FileFormat {
    id: 477,
    source_type: SourceType::Pronom,
    name: "FileMaker Pro Database",
    extensions: &["fp3", "fmp", "fp", "fm"],
    media_types: &["application/x-filemaker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x02,
                        0x00, 0x02, 0xC0,
                    ]),
                    Token::WildcardCount(527),
                    Token::Literal(&[0x50, 0x72, 0x6F, 0x20, 0x33]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
