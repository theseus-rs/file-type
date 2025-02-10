use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862252: FileFormat = FileFormat {
    id: 105_862_252,
    source_type: SourceType::Wikidata,
    name: "MultiSim Design (generic)",
    extensions: &["ms10", "ms8", "ms9"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x4D, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64,
                    0x45, 0x6C, 0x65, 0x63, 0x74, 0x72, 0x6F, 0x6E, 0x69, 0x63, 0x73, 0x57, 0x6F,
                    0x72, 0x6B, 0x62, 0x65, 0x6E, 0x63, 0x68, 0x58, 0x4D, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
