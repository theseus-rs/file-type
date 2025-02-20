use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865009: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_009,
        source_type: SourceType::Wikidata,
        name: "Mission Patch",
        extensions: &["pat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x50, 0x61, 0x74, 0x63,
                        0x68, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
