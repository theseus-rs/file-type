use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850807: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_807,
        source_type: SourceType::Wikidata,
        name: "KOM game data archive",
        extensions: &["kom"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x4F, 0x47, 0x20, 0x47, 0x43, 0x20, 0x54, 0x45, 0x41, 0x4D, 0x20,
                        0x4D, 0x41, 0x53, 0x53, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x56, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
