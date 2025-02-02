use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856132: FileFormat = FileFormat {
    id: 105_856_132,
    source_type: SourceType::Wikidata,
    name: "Touhou Danmakufu script",
    extensions: &["dnh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x54, 0x6F, 0x75, 0x68, 0x6F, 0x75, 0x44, 0x61, 0x6E, 0x6D, 0x61, 0x6B,
                    0x75, 0x66, 0x75, 0x0D, 0x0A, 0x23, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x5B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
