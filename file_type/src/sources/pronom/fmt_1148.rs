use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1148: FileFormat = FileFormat {
    id: 1_957,
    puid: "fmt/1148",
    name: "SIDOUN WinAVA Format",
    extensions: &["swa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x41, 0x56, 0x41, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
