use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851380: FileFormat = FileFormat {
    id: 105_851_380,
    source_type: SourceType::Wikidata,
    name: "Xoom Tutor tutorial (with title)",
    extensions: &["tut"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x74, 0x69, 0x74, 0x6C, 0x65, 0x20, 0x58, 0x6F, 0x6F, 0x6D, 0x54, 0x75, 0x74,
                    0x6F, 0x72, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
