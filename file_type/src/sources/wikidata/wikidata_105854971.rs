use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854971: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_971,
        source_type: SourceType::Wikidata,
        name: "BIX Archiver compressed archive",
        extensions: &["bix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x58, 0x30, 0xC1, 0xB8, 0x03, 0x9A, 0xF1,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
