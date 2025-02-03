use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858790: FileFormat = FileFormat {
    id: 105_858_790,
    source_type: SourceType::Wikidata,
    name: "Memotech MTX BASIC source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF2, 0xF8, 0x59, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
