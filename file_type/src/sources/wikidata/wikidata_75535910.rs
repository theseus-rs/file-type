use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75535910: FileFormat = FileFormat {
    id: 75_535_910,
    source_type: SourceType::Wikidata,
    name: "UP! 3D model",
    extensions: &["up3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x50, 0x20, 0x53, 0x6F, 0x6C, 0x69, 0x64, 0x73, 0x20, 0x2D, 0x2D, 0x20,
                    0x43, 0x53, 0x4D, 0x33, 0x2E, 0x30, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65,
                    0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
