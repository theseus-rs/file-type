use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858402: FileFormat = FileFormat {
    id: 105_858_402,
    source_type: SourceType::Wikidata,
    name: "FRITZ!Box configuration backup",
    extensions: &["export"],
    media_types: &["application/x-avm-export"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x46, 0x52, 0x49, 0x54, 0x5A, 0x21, 0x42, 0x6F,
                    0x78, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
