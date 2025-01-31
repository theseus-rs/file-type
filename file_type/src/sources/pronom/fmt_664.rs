use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_664: FileFormat = FileFormat {
    id: 1_463,
    puid: "fmt/664",
    name: "Gerber Format",
    extensions: &["gbr"],
    media_types: &["application/vnd.gerber"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x46, 0x53, 0x4C, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
