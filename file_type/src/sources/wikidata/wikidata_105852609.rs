use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_609,
        source_type: SourceType::Wikidata,
        name: "SWiSH project",
        extensions: &["swi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x05, 0xFF, 0xFE, 0xFF, 0x13, 0x53, 0x00, 0x57, 0x00, 0x69, 0x00,
                        0x53, 0x00, 0x48, 0x00, 0x6D, 0x00, 0x61, 0x00, 0x78, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
