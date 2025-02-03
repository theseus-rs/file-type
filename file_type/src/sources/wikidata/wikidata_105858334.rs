use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858334: FileFormat = FileFormat {
    id: 105_858_334,
    source_type: SourceType::Wikidata,
    name: "EasyRecovery saved recovery state",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x52, 0x46, 0x53, 0x53, 0x41, 0x56, 0x45, 0x44, 0x41, 0x54, 0x41, 0x46,
                    0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
