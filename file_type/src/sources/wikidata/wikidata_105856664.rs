use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856664: FileFormat = FileFormat {
    id: 105_856_664,
    source_type: SourceType::Wikidata,
    name: "Ulysses Native Format",
    extensions: &["unf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x20, 0x43, 0x6C, 0x61, 0x73, 0x73, 0x3A,
                    0x09, 0x4C, 0x4F, 0x55, 0x44, 0x53, 0x50, 0x45, 0x41, 0x4B, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
