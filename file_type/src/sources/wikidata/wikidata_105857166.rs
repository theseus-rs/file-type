use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857166: FileFormat = FileFormat {
    id: 105_857_166,
    source_type: SourceType::Wikidata,
    name: "ERDAS Imagine Hierarchical File Architecture",
    extensions: &["hfa"],
    media_types: &["application/x-erdas-hfa"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x48, 0x46, 0x41, 0x5F, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5F, 0x54,
                    0x41, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
