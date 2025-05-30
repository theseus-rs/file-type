use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854548: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_548,
        source_type: SourceType::Wikidata,
        name: "Adventure Game Studio saved game",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x64, 0x76, 0x65, 0x6E, 0x74, 0x75, 0x72, 0x65, 0x20, 0x47, 0x61,
                        0x6D, 0x65, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x73, 0x61,
                        0x76, 0x65, 0x64, 0x20, 0x67, 0x61, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
