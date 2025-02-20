use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863416: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_416,
        source_type: SourceType::Wikidata,
        name: "Mnemosyne database",
        extensions: &["mem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x2D, 0x20, 0x4D, 0x6E, 0x65, 0x6D, 0x6F, 0x73, 0x79, 0x6E,
                        0x65, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x42, 0x61, 0x73, 0x65, 0x20,
                        0x2D, 0x2D, 0x2D, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
