use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858635: FileFormat = FileFormat {
    id: 105_858_635,
    source_type: SourceType::Wikidata,
    name: "Accelerated Designs PCB Library",
    extensions: &["bxl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x1A, 0x16, 0x04, 0x50, 0xA8, 0x32, 0x8B, 0x00, 0x8A, 0xC0, 0x3C, 0xBC,
                    0x29, 0x28, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
