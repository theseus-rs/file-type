use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859879: FileFormat = FileFormat {
    id: 105_859_879,
    source_type: SourceType::Wikidata,
    name: "Nokia Saved SMS (Unicode)",
    extensions: &["vmg"],
    media_types: &["text/x-vMessage"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x00, 0x45, 0x00, 0x47, 0x00, 0x49, 0x00, 0x4E, 0x00, 0x3A, 0x00, 0x56,
                    0x00, 0x4D, 0x00, 0x53, 0x00, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
