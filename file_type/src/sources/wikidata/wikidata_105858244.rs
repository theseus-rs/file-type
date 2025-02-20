use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858244: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_244,
        source_type: SourceType::Wikidata,
        name: "Encrypted PhotoList",
        extensions: &["epl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x4C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
