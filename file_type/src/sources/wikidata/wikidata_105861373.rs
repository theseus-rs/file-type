use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861373: FileFormat = FileFormat {
    id: 105_861_373,
    source_type: SourceType::Wikidata,
    name: "Linux Software Map entry (v3)",
    extensions: &["lsm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
