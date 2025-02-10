use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_020,
        source_type: SourceType::Wikidata,
        name: "Norton Disk Doctor UnDo file",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4E, 0x43, 0x49, 0x55, 0x4E, 0x44, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
