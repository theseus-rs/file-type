use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866317: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_317,
        source_type: SourceType::Wikidata,
        name: "Commodore 64 PCLINK container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0xCD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
