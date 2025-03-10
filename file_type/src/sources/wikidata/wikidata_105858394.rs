use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858394: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_394,
        source_type: SourceType::Wikidata,
        name: "Experimental archiver compressed data (v1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31,
                        0x32, 0x33, 0x34, 0x35, 0x42, 0x5A, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26,
                        0x53, 0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
