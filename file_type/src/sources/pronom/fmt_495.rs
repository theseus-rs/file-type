use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_495: FileFormat = FileFormat {
    id: 1_282,
    puid: "fmt/495",
    name: "ATCO-CIF",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x54, 0x43, 0x4F, 0x2D, 0x43, 0x49, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
