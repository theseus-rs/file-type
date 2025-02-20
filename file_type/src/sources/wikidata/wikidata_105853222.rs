use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853222: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_222,
        source_type: SourceType::Wikidata,
        name: "TradeWinds 2 savegame",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x57, 0x32, 0x20, 0x53, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6D, 0x65,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
