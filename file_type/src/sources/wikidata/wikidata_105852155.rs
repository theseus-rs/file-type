use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852155: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_155,
        source_type: SourceType::Wikidata,
        name: "Windows Shadow spooler (2000/XP)",
        extensions: &["shd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x49, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
