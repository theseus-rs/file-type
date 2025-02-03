use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853409: FileFormat = FileFormat {
    id: 105_853_409,
    source_type: SourceType::Wikidata,
    name: "Standard Test and Programming Language",
    extensions: &["stapl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x4F, 0x54, 0x45, 0x20, 0x22, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4F, 0x52,
                    0x22, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
