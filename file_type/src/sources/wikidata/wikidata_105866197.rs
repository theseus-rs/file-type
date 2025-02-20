use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866197: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_197,
        source_type: SourceType::Wikidata,
        name: "Massive Development package/container",
        extensions: &["pak"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x46, 0x49, 0x4C, 0x45, 0x00,
                        0x03, 0x00, 0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
