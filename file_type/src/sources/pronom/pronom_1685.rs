use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1685: FileFormat = FileFormat {
    id: 1_685,
    source_type: SourceType::Pronom,
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
