use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852185: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_185,
        source_type: SourceType::Wikidata,
        name: "SafeBoot encrypted data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x61, 0x66, 0x65, 0x42, 0x6F, 0x6F, 0x74, 0x45, 0x6E, 0x63, 0x46,
                        0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
