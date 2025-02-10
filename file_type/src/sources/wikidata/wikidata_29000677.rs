use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29000677: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_677,
        source_type: SourceType::Wikidata,
        name: "Yet Another Object Description Language",
        extensions: &["ydl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x59, 0x41, 0x4F, 0x44, 0x4C, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
