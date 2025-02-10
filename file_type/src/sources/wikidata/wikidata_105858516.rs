use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_516,
        source_type: SourceType::Wikidata,
        name: "HRU bitmap",
        extensions: &["hru"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x35, 0x4B, 0x50, 0x35, 0x31, 0x5D, 0x2A, 0x67, 0x72, 0x72, 0x80, 0x83,
                        0x85, 0x63, 0x7A, 0x7D, 0x6B, 0x43, 0x6A, 0x55, 0x49, 0x53, 0x64, 0x4F,
                        0x51, 0x61, 0x30, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
