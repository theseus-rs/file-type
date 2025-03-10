use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858638: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_638,
        source_type: SourceType::Wikidata,
        name: "BD4 save game (v1)",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x44, 0x34, 0x5F, 0x53, 0x41, 0x56, 0x45, 0x5F, 0x56, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
