use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855088: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_088,
        source_type: SourceType::Wikidata,
        name: "Amiga Money categories (v1)",
        extensions: &["amm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x4D, 0x31, 0x43, 0x41, 0x54, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
