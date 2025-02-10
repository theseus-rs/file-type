use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858985: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_985,
        source_type: SourceType::Wikidata,
        name: "Wii Effect controls",
        extensions: &["breff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x45, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
