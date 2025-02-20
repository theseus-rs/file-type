use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851448: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_448,
        source_type: SourceType::Wikidata,
        name: "Jane's Longbow 2 game data archive",
        extensions: &["tre"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4B, 0x4E, 0x4B, 0x54, 0x52, 0x45, 0x45, 0x30, 0x31, 0x30, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
