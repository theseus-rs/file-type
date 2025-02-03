use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858576: FileFormat = FileFormat {
    id: 105_858_576,
    source_type: SourceType::Wikidata,
    name: "TNC Packed Backup",
    extensions: &["bck"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x4E, 0x43, 0x20, 0x50, 0x41, 0x43, 0x4B, 0x45, 0x44, 0x20, 0x42, 0x41,
                    0x43, 0x4B, 0x55, 0x50, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x2D, 0x20, 0x56,
                    0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
