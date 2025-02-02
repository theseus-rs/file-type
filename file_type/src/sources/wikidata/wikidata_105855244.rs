use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855244: FileFormat = FileFormat {
    id: 105_855_244,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Font",
    extensions: &["fnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x45, 0x4C, 0x55, 0x58, 0x45, 0x2E, 0x46, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
