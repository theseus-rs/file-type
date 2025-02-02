use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115331958: FileFormat = FileFormat {
    id: 115_331_958,
    source_type: SourceType::Wikidata,
    name: "Premiere project, version 1.0 - 4.2",
    extensions: &["ppj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Any(&[
                    &[Token::Literal(&[0x52, 0x61, 0x6E, 0x64])],
                    &[Token::Literal(&[0x64, 0x6E, 0x61, 0x52])],
                ])],
            },
        }],
    }],
    related_formats: &[],
};
