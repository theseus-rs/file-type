use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1971: FileFormat = FileFormat {
    id: 1_971,
    source_type: SourceType::Pronom,
    name: "Folio Shadow File",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(224),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x04, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[
                        0x01, 0x00, 0x00, 0x00, 0xFC, 0xAE, 0x56, 0x89, 0x62, 0x74, 0xBF, 0xAE,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
