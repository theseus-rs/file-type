use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762833: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_833,
        source_type: SourceType::Wikidata,
        name: "XG5000 Project",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x47, 0x35, 0x30, 0x30, 0x30, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                        0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
