use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857533: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_533,
        source_type: SourceType::Wikidata,
        name: "ScreenBuilder image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFB, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
