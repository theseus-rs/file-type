use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856975: FileFormat = FileFormat {
    id: 105_856_975,
    source_type: SourceType::Wikidata,
    name: "Beaver Sweeper module",
    extensions: &["gtk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2B, 0x53, 0x4E, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
