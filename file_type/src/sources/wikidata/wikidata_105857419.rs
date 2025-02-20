use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857419: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_419,
        source_type: SourceType::Wikidata,
        name: "Binary ExtendScript Script",
        extensions: &["jsxbin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x4A, 0x53, 0x58, 0x42, 0x49, 0x4E, 0x40, 0x45, 0x53, 0x40,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
