use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858128: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_128,
        source_type: SourceType::Wikidata,
        name: "Card Image Backup format",
        extensions: &["cib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x43, 0x49, 0x42, 0x3E, 0xC1, 0xB0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
