use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852198: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_198,
        source_type: SourceType::Wikidata,
        name: "Spring Engine Map",
        extensions: &["smf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x70, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x6D, 0x61, 0x70, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
