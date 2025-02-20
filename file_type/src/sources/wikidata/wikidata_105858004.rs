use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858004: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_004,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere cache",
        extensions: &["ims"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x00, 0x22, 0x00, 0x6D, 0x00, 0x41, 0x00, 0x6C, 0x00, 0x77, 0x00,
                        0x61, 0x00, 0x79, 0x00, 0x73, 0x00, 0x55, 0x00, 0x6E, 0x00, 0x71, 0x00,
                        0x75, 0x00, 0x69, 0x00, 0x65, 0x00, 0x74, 0x00, 0x22, 0x00, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
