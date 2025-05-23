use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859613: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_613,
        source_type: SourceType::Wikidata,
        name: "Valve Choreography Data format",
        extensions: &["vcd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x43, 0x68, 0x6F, 0x72, 0x65, 0x6F, 0x20, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
