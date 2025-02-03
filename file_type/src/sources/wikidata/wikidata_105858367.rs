use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858367: FileFormat = FileFormat {
    id: 105_858_367,
    source_type: SourceType::Wikidata,
    name: "ExamView Online Test",
    extensions: &["eot"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1A, 0x45, 0x56, 0x4F, 0x4E, 0x4C, 0x49, 0x4E, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
