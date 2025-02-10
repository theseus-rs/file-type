use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866196: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_196,
        source_type: SourceType::Wikidata,
        name: "OS/2 Pointer (color)",
        extensions: &["ptr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
