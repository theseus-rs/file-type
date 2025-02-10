use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861147: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_147,
        source_type: SourceType::Wikidata,
        name: "Lotus Word Pro document (v7)",
        extensions: &["lwp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x6F, 0x72, 0x64, 0x50, 0x72, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x4C, 0x57, 0x50, 0x37,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
