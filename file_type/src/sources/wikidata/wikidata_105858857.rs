use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858857: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_857,
        source_type: SourceType::Wikidata,
        name: "Roku Base Index Frames container",
        extensions: &["bif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x42, 0x49, 0x46, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
