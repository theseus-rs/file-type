use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853983: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_983,
        source_type: SourceType::Wikidata,
        name: "Ami-Back Backup",
        extensions: &["bac"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x41, 0x43, 0x4B, 0x4D, 0x49, 0x4B, 0x45, 0x44, 0x49, 0x53, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
