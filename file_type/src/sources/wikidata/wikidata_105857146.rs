use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857146: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_146,
        source_type: SourceType::Wikidata,
        name: "Applause Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x50, 0x50, 0x4C, 0x41, 0x55, 0x53, 0x45, 0x20, 0x48, 0x45, 0x4C,
                        0x50, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
