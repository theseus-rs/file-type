use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860423: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_423,
        source_type: SourceType::Wikidata,
        name: "Rune map",
        extensions: &["run"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xC1, 0x83, 0x2A, 0x9E, 0x45, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
