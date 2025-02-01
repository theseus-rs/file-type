use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1139: FileFormat = FileFormat {
    id: 1_949,
    puid: "fmt/1139",
    name: "VectorWorks",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(14),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20, 0x39,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
