use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851473: FileFormat = FileFormat {
    id: 105_851_473,
    source_type: SourceType::Wikidata,
    name: "GObject introspection Type Libray data",
    extensions: &["typelib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4F, 0x42, 0x4A, 0x0A, 0x4D, 0x45, 0x54, 0x41, 0x44, 0x41, 0x54, 0x41,
                    0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
