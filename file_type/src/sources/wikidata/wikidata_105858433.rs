use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858433: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_433,
        source_type: SourceType::Wikidata,
        name: "EDI Install LZS compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x44, 0x49, 0x4C, 0x5A, 0x53, 0x53, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
