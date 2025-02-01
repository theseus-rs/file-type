use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1284: FileFormat = FileFormat {
    id: 2_102,
    puid: "fmt/1284",
    name: "PFS:First Choice Database",
    extensions: &["fol"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
