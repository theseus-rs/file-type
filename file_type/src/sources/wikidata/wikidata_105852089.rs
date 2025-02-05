use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852089: FileFormat = FileFormat {
    id: 105_852_089,
    source_type: SourceType::Wikidata,
    name: "Solace Virtual Tape format 1",
    extensions: &["svt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x56, 0x54, 0x31, 0x3A, 0x20, 0x53, 0x6F, 0x6C, 0x61, 0x63, 0x65, 0x20,
                    0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x54, 0x61, 0x70, 0x65, 0x20,
                    0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
