use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856479: FileFormat = FileFormat {
    id: 105_856_479,
    source_type: SourceType::Wikidata,
    name: "WinZip Job File",
    extensions: &["wjf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
