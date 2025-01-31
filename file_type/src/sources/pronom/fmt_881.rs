use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_881: FileFormat = FileFormat {
    id: 1_685,
    puid: "fmt/881",
    name: "Microsoft Document Imaging File Format",
    extensions: &["mdi"],
    media_types: &["image/vnd.ms-modi"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x2A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
