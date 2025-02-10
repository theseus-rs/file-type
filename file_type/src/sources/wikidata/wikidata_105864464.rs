use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864464: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_464,
        source_type: SourceType::Wikidata,
        name: "Applause Palette",
        extensions: &["pal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x6C, 0x65, 0x74, 0x74, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
