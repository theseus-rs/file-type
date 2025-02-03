use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859059: FileFormat = FileFormat {
    id: 105_859_059,
    source_type: SourceType::Wikidata,
    name: "Warrior Kings game data",
    extensions: &["bcp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x4B, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x32, 0x2E, 0x30, 0x31,
                    0x20, 0x28, 0x63, 0x29, 0x20, 0x42, 0x6C, 0x61, 0x63, 0x6B, 0x20, 0x43, 0x61,
                    0x63, 0x74, 0x75, 0x73, 0x20, 0x47, 0x61, 0x6D, 0x65, 0x73, 0x20, 0x4C, 0x69,
                    0x6D, 0x69, 0x74, 0x65, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
