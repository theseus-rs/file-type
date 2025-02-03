use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862444: FileFormat = FileFormat {
    id: 105_862_444,
    source_type: SourceType::Wikidata,
    name: "Norton Commander module message (ENG)",
    extensions: &["msg"],
    media_types: &["application/x-norton-msg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x62, 0x6F, 0x72, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
