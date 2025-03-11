use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860503: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_503,
        source_type: SourceType::Wikidata,
        name: "BIS raP encoded format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x72, 0x61, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
