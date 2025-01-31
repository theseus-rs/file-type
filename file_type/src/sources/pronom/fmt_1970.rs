use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1970: FileFormat = FileFormat {
    id: 2_836,
    puid: "fmt/1970",
    name: "MOXCEL",
    extensions: &["mxl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x58, 0x43, 0x45, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
