use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_322: FileFormat = FileFormat {
    id: 1_067,
    puid: "fmt/322",
    name: "Portable Form File",
    extensions: &["pff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x46, 0x46, 0x00, 0x07, 0x02, 0x00, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
