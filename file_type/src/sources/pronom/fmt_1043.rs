use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1043: FileFormat = FileFormat {
    id: 1_848,
    puid: "fmt/1043",
    name: "Microsoft PRX File",
    extensions: &["prx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x3C, 0x00, 0x50, 0x00, 0x52, 0x00, 0x58, 0x00, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
