use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858913: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_913,
        source_type: SourceType::Wikidata,
        name: "Artlantis Billboard",
        extensions: &["bb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x41, 0x72, 0x63, 0x68, 0x69, 0x63, 0x61, 0x64, 0x6F, 0x2E, 0x42,
                        0x69, 0x6C, 0x6C, 0x62, 0x6F, 0x61, 0x72, 0x64, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
