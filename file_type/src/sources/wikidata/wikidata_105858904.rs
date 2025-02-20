use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858904: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_904,
        source_type: SourceType::Wikidata,
        name: "Compressed Champions' Interlace bitmap",
        extensions: &["cci"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x49, 0x4E, 0x20, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
