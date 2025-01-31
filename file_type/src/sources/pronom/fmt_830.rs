use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_830: FileFormat = FileFormat {
    id: 1_631,
    puid: "fmt/830",
    name: "Qsplat Model",
    extensions: &["qs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x53, 0x70, 0x6C, 0x61, 0x74, 0x31, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
