use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_093,
        source_type: SourceType::Wikidata,
        name: "Sonic Arranger module (v1.0)",
        extensions: &["sa"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4F, 0x41, 0x52, 0x56, 0x31, 0x2E, 0x30, 0x53, 0x54, 0x42, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
