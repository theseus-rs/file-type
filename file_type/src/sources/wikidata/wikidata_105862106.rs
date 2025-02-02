use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862106: FileFormat = FileFormat {
    id: 105_862_106,
    source_type: SourceType::Wikidata,
    name: "Platinen Layout Programm Macro",
    extensions: &["mac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4C, 0x50, 0x00, 0x00, 0x82, 0x4D, 0x41, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
