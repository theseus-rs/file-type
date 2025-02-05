use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862641: FileFormat = FileFormat {
    id: 105_862_641,
    source_type: SourceType::Wikidata,
    name: "MasterCook Cookbook File",
    extensions: &["mcf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6F, 0x6F, 0x6B, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
