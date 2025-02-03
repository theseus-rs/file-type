use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856776: FileFormat = FileFormat {
    id: 105_856_776,
    source_type: SourceType::Wikidata,
    name: "MikMod module",
    extensions: &["uni"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
