use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856073: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_073,
        source_type: SourceType::Wikidata,
        name: "CaveStory save game",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x6F, 0x30, 0x34, 0x31, 0x32, 0x32, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
