use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1437: FileFormat = FileFormat {
    id: 1_437,
    source_type: SourceType::Pronom,
    name: "Microsoft OneNote",
    extensions: &["one"],
    media_types: &["application/msonenote"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xE4, 0x52, 0x5C, 0x7B, 0x8C, 0xD8, 0xA7, 0x4D, 0xAE, 0xB1, 0x53, 0x78, 0xD0,
                    0x29, 0x96, 0xD3,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
