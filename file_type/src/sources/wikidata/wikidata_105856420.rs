use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856420: FileFormat = FileFormat {
    id: 105_856_420,
    source_type: SourceType::Wikidata,
    name: "Whirlwind game data Package (v2.1)",
    extensions: &["wpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x68, 0x69, 0x72, 0x6C, 0x77, 0x69, 0x6E, 0x64, 0x20, 0x32, 0x2E, 0x31,
                    0x20, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
