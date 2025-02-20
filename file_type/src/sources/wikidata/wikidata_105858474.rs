use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858474: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_474,
        source_type: SourceType::Wikidata,
        name: "DCMO6 emulator tape image",
        extensions: &["k7"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x43, 0x4D, 0x4F, 0x36, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                        0x01, 0x01, 0x01, 0x01, 0x3C, 0x5A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
