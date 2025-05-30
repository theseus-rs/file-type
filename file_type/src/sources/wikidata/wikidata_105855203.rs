use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855203: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_203,
        source_type: SourceType::Wikidata,
        name: "DynaCADD vector Font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x59, 0x4E, 0x41, 0x56, 0x45, 0x43, 0x54, 0x46, 0x4E, 0x54, 0x00,
                        0x44, 0x79, 0x6E, 0x61, 0x43, 0x41, 0x44, 0x44, 0x20, 0x56, 0x65, 0x63,
                        0x74, 0x6F, 0x72, 0x20, 0x46, 0x6F, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
