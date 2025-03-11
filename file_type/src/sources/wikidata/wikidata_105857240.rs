use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857240: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_240,
        source_type: SourceType::Wikidata,
        name: "HP 49 series binary transfer data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x50, 0x48, 0x50, 0x34, 0x39, 0x2D, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
