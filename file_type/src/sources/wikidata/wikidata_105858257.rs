use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858257: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_257,
        source_type: SourceType::Wikidata,
        name: "BeIA Compressed ELF executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x43, 0x45, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
