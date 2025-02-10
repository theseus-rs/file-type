use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858429: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_429,
        source_type: SourceType::Wikidata,
        name: "Windows Event Viewer Log",
        extensions: &["evt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x30, 0x00, 0x00, 0x00, 0x4C, 0x66, 0x4C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
