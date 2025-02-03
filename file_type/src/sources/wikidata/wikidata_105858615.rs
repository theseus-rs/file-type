use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858615: FileFormat = FileFormat {
    id: 105_858_615,
    source_type: SourceType::Wikidata,
    name: "Adrenaline Rush Hour game data",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x53, 0x50, 0x53, 0x64, 0x75, 0x6D, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
