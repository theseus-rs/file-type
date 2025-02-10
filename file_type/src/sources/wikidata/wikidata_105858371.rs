use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858371: FileFormat = FileFormat {
    id: 105_858_371,
    source_type: SourceType::Wikidata,
    name: "EasyPlot save file",
    extensions: &["ep", "epw", "ezp", "txt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x45, 0x61, 0x73, 0x79,
                    0x50, 0x6C, 0x6F, 0x74, 0x20, 0x73, 0x61, 0x76, 0x65, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x20, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
