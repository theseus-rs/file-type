use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67441966: FileType = FileType {
    file_format: &FileFormat {
        id: 67_441_966,
        source_type: SourceType::Wikidata,
        name: "Robocode Battle",
        extensions: &["battle"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x42, 0x61, 0x74, 0x74, 0x6C, 0x65, 0x20, 0x50, 0x72, 0x6F, 0x70,
                        0x65, 0x72, 0x74, 0x69, 0x65, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
