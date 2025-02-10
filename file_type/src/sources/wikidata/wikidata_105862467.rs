use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862467: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_467,
        source_type: SourceType::Wikidata,
        name: "StarTrekker 4-channel module",
        extensions: &["mod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x54, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
