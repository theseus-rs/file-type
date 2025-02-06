use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858264: FileFormat = FileFormat {
    id: 105_858_264,
    source_type: SourceType::Wikidata,
    name: "EDIF Netlist",
    extensions: &["edn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x65, 0x64, 0x69, 0x66, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
