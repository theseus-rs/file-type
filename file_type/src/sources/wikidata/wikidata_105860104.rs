use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860104: FileFormat = FileFormat {
    id: 105_860_104,
    source_type: SourceType::Wikidata,
    name: "ViPlay Subtitle Format",
    extensions: &["vsf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x2A, 0x20, 0x56, 0x49, 0x50, 0x4C, 0x41, 0x59, 0x20, 0x53, 0x55, 0x42,
                    0x54, 0x49, 0x54, 0x4C, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x2A, 0x7D,
                    0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
