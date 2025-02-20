use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864827: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_827,
        source_type: SourceType::Wikidata,
        name: "Rebels: Prison Escape game data",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x6F, 0x4E, 0x75, 0x54, 0x3A, 0x46, 0x53, 0x50, 0x61, 0x63, 0x6B,
                        0x65, 0x64, 0x3A, 0x30, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
