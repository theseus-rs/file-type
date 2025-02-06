use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2383: FileFormat = FileFormat {
    id: 2_383,
    source_type: SourceType::Pronom,
    name: "SelF-eXtracting LHA/LZH Compressed Files",
    extensions: &["sfx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(3_000),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x36, 0x34, 0x2F, 0x43, 0x31, 0x32, 0x38, 0x20, 0x53, 0x45, 0x4C, 0x46,
                    0x20, 0x45, 0x58, 0x54, 0x52, 0x41, 0x43, 0x54, 0x49, 0x4E, 0x47, 0x20, 0x4C,
                    0x48, 0x41, 0x52, 0x43, 0x48, 0x49, 0x56, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
