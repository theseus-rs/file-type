use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_601,
        source_type: SourceType::Wikidata,
        name: "SLIM compressed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCD, 0x20, 0x73, 0x4C, 0x69, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
