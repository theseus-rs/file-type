use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867551: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_551,
        source_type: SourceType::Wikidata,
        name: "Nullsoft Database Engine Index",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x44, 0x45, 0x49, 0x4E, 0x44, 0x45, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
