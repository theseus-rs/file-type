use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852303: FileFormat = FileFormat {
    id: 105_852_303,
    source_type: SourceType::Wikidata,
    name: "EnVision Publisher DTP Font",
    extensions: &["svf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x83, 0xF9, 0xF8, 0xF8, 0xF9, 0xF9, 0xF9, 0x11, 0xFA, 0xF8, 0xF9, 0xF8,
                    0xF9, 0xD9, 0xF9, 0x19, 0xF9,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
