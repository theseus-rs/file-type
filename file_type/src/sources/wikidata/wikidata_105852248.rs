use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852248: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_248,
        source_type: SourceType::Wikidata,
        name: "Silver Scene",
        extensions: &["scr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x49, 0x4C, 0x56, 0x45, 0x52, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
