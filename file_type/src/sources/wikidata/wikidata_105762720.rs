use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_720,
        source_type: SourceType::Wikidata,
        name: "XPilot NG map",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x58, 0x50, 0x69, 0x6C, 0x6F, 0x74, 0x4D, 0x61, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
