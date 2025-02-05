use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858956: FileFormat = FileFormat {
    id: 105_858_956,
    source_type: SourceType::Wikidata,
    name: "DEC SIXEL Graphic bitmap",
    extensions: &["six"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x90, 0x30, 0x3B, 0x30, 0x3B, 0x38, 0x71, 0x22, 0x31, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
