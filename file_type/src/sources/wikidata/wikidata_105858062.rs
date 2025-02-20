use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858062: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_062,
        source_type: SourceType::Wikidata,
        name: "Virtual98 harddisk image",
        extensions: &["hdd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x48, 0x44, 0x31, 0x2E, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
