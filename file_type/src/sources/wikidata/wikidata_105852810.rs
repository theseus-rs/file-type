use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852810: FileFormat = FileFormat {
    id: 105_852_810,
    source_type: SourceType::Wikidata,
    name: "SketchUp model",
    extensions: &["skp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0xFF, 0x0E, 0x53, 0x00, 0x6B, 0x00, 0x65, 0x00, 0x74, 0x00, 0x63,
                    0x00, 0x68, 0x00, 0x55, 0x00, 0x70, 0x00, 0x20, 0x00, 0x4D, 0x00, 0x6F, 0x00,
                    0x64, 0x00, 0x65, 0x00, 0x6C, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
