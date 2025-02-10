use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856026: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_026,
        source_type: SourceType::Wikidata,
        name: "Dungeon Siege 2 data",
        extensions: &["ds2res"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x53, 0x67, 0x32, 0x54, 0x61, 0x6E, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
