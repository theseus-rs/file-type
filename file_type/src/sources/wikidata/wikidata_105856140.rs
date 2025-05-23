use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856140: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_140,
        source_type: SourceType::Wikidata,
        name: "ISIS Schematic file",
        extensions: &["dsn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x53, 0x49, 0x53, 0x20, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x41, 0x54,
                        0x49, 0x43, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
