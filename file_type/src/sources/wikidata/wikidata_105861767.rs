use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861767: FileFormat = FileFormat {
    id: 105_861_767,
    source_type: SourceType::Wikidata,
    name: "blueMSX Cheats Format",
    extensions: &["mcf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x63, 0x68, 0x65, 0x61, 0x74, 0x73, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x62,
                    0x6C, 0x75, 0x65, 0x4D, 0x53, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
