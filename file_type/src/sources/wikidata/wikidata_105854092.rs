use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854092: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_092,
        source_type: SourceType::Wikidata,
        name: "ADPACK32 compressed archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x44, 0x50, 0x41, 0x43, 0x4B, 0x33, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
