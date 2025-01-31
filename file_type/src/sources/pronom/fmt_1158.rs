use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1158: FileFormat = FileFormat {
    id: 1_968,
    puid: "fmt/1158",
    name: "Folio Infobase File",
    extensions: &["nfo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(212),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xCD, 0xAB, 0x89, 0x00, 0x00, 0x03, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
