use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851637: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_637,
        source_type: SourceType::Wikidata,
        name: "Tor state",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x54, 0x6F, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x20, 0x6C, 0x61, 0x73, 0x74, 0x20, 0x67, 0x65,
                        0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
