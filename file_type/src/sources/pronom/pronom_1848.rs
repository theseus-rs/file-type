use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1848: FileFormat = FileFormat {
    id: 1_848,
    source_type: SourceType::Pronom,
    name: "Microsoft PRX File",
    extensions: &["prx"],
    media_types: &[],
    signatures: &[Signature {
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
