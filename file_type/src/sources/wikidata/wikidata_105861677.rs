use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861677: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_677,
        source_type: SourceType::Wikidata,
        name: "UltraDefrag Lua Report",
        extensions: &["luar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x20, 0x55, 0x6C, 0x74, 0x72, 0x61, 0x44, 0x65, 0x66, 0x72,
                        0x61, 0x67, 0x20, 0x72, 0x65, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x66, 0x6F,
                        0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
