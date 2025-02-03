use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860639: FileFormat = FileFormat {
    id: 105_860_639,
    source_type: SourceType::Wikidata,
    name: "Regressi Win data",
    extensions: &["rw3"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x56, 0x41, 0x52, 0x49, 0x53, 0x54, 0x45, 0x20, 0x52, 0x45, 0x47, 0x52,
                    0x45, 0x53, 0x53, 0x49, 0x20, 0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57, 0x53, 0x20,
                    0x31, 0x2E, 0x30, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
