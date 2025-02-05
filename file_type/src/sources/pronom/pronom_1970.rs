use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1970: FileFormat = FileFormat {
    id: 1_970,
    source_type: SourceType::Pronom,
    name: "Folio Shadow File",
    extensions: &["sdw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(212),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xCD, 0xAB, 0x89, 0x01, 0x00, 0x03, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
