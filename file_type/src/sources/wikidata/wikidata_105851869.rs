use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851869: FileFormat = FileFormat {
    id: 105_851_869,
    source_type: SourceType::Wikidata,
    name: "ShowBiZ project",
    extensions: &["sbz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x10, 0x04, 0x00, 0x00, 0xF1, 0xC3, 0xB6, 0x8A, 0x21, 0x03, 0x03, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
