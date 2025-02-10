use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852855: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_855,
        source_type: SourceType::Wikidata,
        name: "FrontDesigner Scale setting",
        extensions: &["scl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x61, 0x72, 0x61, 0x6D, 0x65, 0x74, 0x65, 0x72, 0x5D, 0x0D,
                        0x0A, 0x53, 0x74, 0x79, 0x6C, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
