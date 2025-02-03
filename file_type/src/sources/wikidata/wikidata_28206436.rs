use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206436: FileFormat = FileFormat {
    id: 28_206_436,
    source_type: SourceType::Wikidata,
    name: "JPS",
    extensions: &["jps"],
    media_types: &["image/jpeg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
