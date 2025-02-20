use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857639: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_639,
        source_type: SourceType::Wikidata,
        name: "CopyTape intermediate data format",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x54, 0x50, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
