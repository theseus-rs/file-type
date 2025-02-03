use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860693: FileFormat = FileFormat {
    id: 105_860_693,
    source_type: SourceType::Wikidata,
    name: "Grand Theft Auto: San Andreas replay",
    extensions: &["rep"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x74, 0x61, 0x53, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
