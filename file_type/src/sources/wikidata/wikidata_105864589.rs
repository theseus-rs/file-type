use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864589: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_589,
        source_type: SourceType::Wikidata,
        name: "CorelDRAW Pattern (v2.0)",
        extensions: &["pat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4C, 0x6C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
