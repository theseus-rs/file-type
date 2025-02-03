use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849660: FileFormat = FileFormat {
    id: 105_849_660,
    source_type: SourceType::Wikidata,
    name: "42's Core War compiled program",
    extensions: &["cor"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0xEA, 0x83, 0xF3])],
            },
        }],
    }],
    related_formats: &[],
};
