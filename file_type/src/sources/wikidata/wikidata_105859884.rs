use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859884: FileFormat = FileFormat {
    id: 105_859_884,
    source_type: SourceType::Wikidata,
    name: "Nokia Saved SMS",
    extensions: &["vmg"],
    media_types: &["text/x-vMessage"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x4D, 0x53, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
