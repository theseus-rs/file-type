use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855226: FileFormat = FileFormat {
    id: 105_855_226,
    source_type: SourceType::Wikidata,
    name: "FontCreator Project",
    extensions: &["fcp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x89, 0x46, 0x43, 0x50, 0x0D, 0x0A, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
