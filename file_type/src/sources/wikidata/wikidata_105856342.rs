use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856342: FileFormat = FileFormat {
    id: 105_856_342,
    source_type: SourceType::Wikidata,
    name: "Skype user data",
    extensions: &["dbb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x33, 0x33, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
