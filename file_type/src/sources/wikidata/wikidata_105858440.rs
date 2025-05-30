use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858440: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_440,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts LIB container",
        extensions: &["lib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x41, 0x4C, 0x49, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
