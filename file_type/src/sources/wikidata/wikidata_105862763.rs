use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862763: FileFormat = FileFormat {
    id: 105_862_763,
    source_type: SourceType::Wikidata,
    name: "StarWriter keyboard Macro",
    extensions: &["mac"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x5C, 0x5C, 0x5C, 0x20, 0x46, 0x4C, 0x4F, 0x53, 0x4B, 0x45, 0x4C, 0x4E,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
